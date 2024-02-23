use bevy::prelude::*;
use crate::laxer::LaxerFly;
use crate::physics::{Impulse, Torque};
use rand::Rng;

pub struct TargetPlugin;

#[derive(Component)]
pub struct Target(u32);

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (
                move_targets,
                check_collisions,
            ));
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // Floaty cube things
    let mut rng = rand::thread_rng();
    let car = assets.load("renault_logan_2004.glb#Scene0");

    for i in 1..50 {
        let t = Transform::from_xyz(
            rng.gen::<f32>() * 300.0 - 150.0,
            rng.gen::<f32>() * 100.0,
            rng.gen::<f32>() * 300.0 - 150.0
        );

        commands.spawn((
            SceneBundle {
                scene: car.clone(),
                transform: t,
                ..default()
            },
            Target(i),
            Impulse::new(),
            Torque::new()
        ));
    }
}

fn move_targets (
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Target, &mut Impulse, &mut Torque)>)
{
    let dt = time.delta_seconds();
    let elapsed = time.elapsed_seconds();

    let mut rng = rand::thread_rng();


    for (mut tr, t, mut imp, mut tor) in q.iter_mut() {
        let forward = tr.forward();
        let i = t.0 as f32;
        tr.translation += forward * dt *
                (elapsed * (2.0 + i) * 0.01) *
            ((i - 50.0) * 0.1);

        if rng.gen::<f32>() * 100.0 < 0.5 {
            imp.add_force(Vec3::new(
                rng.gen::<f32>() - 0.5,
                rng.gen::<f32>() - 0.5,
                rng.gen::<f32>() - 0.5
                    ).normalize() * 0.1);
        }

        if rng.gen::<f32>() * 100.0 < 0.5 {
            tor.add_force(Vec3::new(
                0.,
                rng.gen::<f32>() - 0.5,
                0.
                    ).normalize() * 0.1);
        }
    }
}

fn check_collisions (
    mut cmds: Commands,
    lax: Query<(Entity, &Transform), With<LaxerFly>>,
    q: Query<(Entity, &Transform), With<Target>>)
{
    for (le, lt) in lax.iter() {
        for (te, tt) in q.iter() {
            if lt.translation.distance(tt.translation) < 4.0 {
                cmds.entity(te).despawn_recursive();
                cmds.entity(le).despawn_recursive();
                continue;
            }
        }
    }
}
