use bevy::prelude::*;
use crate::laxer::LaxerFly;
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Floaty cube things
    let mut rng = rand::thread_rng();
    for i in 1..100 {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid {half_size: Vec3::new(
                    rng.gen::<f32>() * 5.0 + 0.5,
                    rng.gen::<f32>() * 2.0 + 0.1,
                    rng.gen::<f32>() * 3.0 + 0.2
                )})),
                material: materials.add(Color::rgb_u8(255, 144, 55)),
                transform: Transform::from_xyz(
                    rng.gen::<f32>() * 300.0 - 150.0,
                    rng.gen::<f32>() * 100.0,
                    rng.gen::<f32>() * 300.0 - 150.0
                ),
                ..default()
            },
            Target(i)
        ));
    }
}

fn move_targets (
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Target)>)
{
    let dt = time.delta_seconds();
    let elapsed = time.elapsed_seconds();

    for (mut tr, t) in q.iter_mut() {
        let right = tr.right();
        let i = t.0 as f32;
        tr.translation += right * dt *

                (elapsed * (2.0 + i) * 0.01).sin() *
                    ((i - 50.0) * 0.1);
    }
}

fn check_collisions (
    mut cmds: Commands,
    lax: Query<(Entity, &Transform), With<LaxerFly>>,
    q: Query<(Entity, &Transform), With<Target>>)
{
    for (le, lt) in lax.iter() {
        for (te, tt) in q.iter() {
            if lt.translation.distance(tt.translation) < 3.0 {
                cmds.entity(te).despawn();
                cmds.entity(le).despawn_recursive();
                continue;
            }
        }
    }
}
