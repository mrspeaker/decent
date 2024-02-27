use bevy::prelude::*;
use crate::camera::TitleText;
use crate::laxer::LaxerFly;
use crate::physics::{Impulse, Torque};
use crate::particles::Explosion;
use crate::player::RayHit;
use rand::Rng;
use self::Adornment::*;

pub struct TargetPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adornment {
    FunnyHat,
    Sunnies,
    ExtraLimb,
    Umbrella,
    RedScarf,
    FakeBeard,
    NoShirt,
    FlipFlops
}

impl Adornment {
    pub fn iter() -> impl Iterator<Item = Adornment> {
        [FunnyHat, Sunnies, ExtraLimb, Umbrella,
         RedScarf, FakeBeard, NoShirt, FlipFlops].iter().copied()
    }
}

#[derive(Component)]
pub struct Target {
    id: u32,
    perp: bool,
    outfit: [Adornment; 3]
}


impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (
                move_targets,
                check_collisions,
                got_ray_hit
            ));
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // Floaty cube things
    let mut rng = rand::thread_rng();
    let car_one = assets.load("renault_logan_2004.glb#Scene0");
    let car_two = assets.load("car3.glb#Scene0");
    //let car_two = assets.load("car.glb#Scene0");

    let ads: Vec<_> = Adornment::iter().collect();
    let len = ads.len();
    let mut id: u32 = 0;

    for i in 0..len {
        for j in i+1..len {
            for k in j+1..len {
                let mut t = Transform::from_xyz(
                    rng.gen::<f32>() * 300.0 - 150.0,
                    rng.gen::<f32>() * 100.0,
                    rng.gen::<f32>() * 300.0 - 150.0
                );
                t.rotate_y(id as f32 * 37.0);

                let outfit = [
                    ads[i],
                    ads[j],
                    ads[k]
                ];

                //println!("{:?} {:?}", id, outfit);

                commands.spawn((
                    SceneBundle {
                        scene: if i < 25 { car_one.clone() } else { car_two.clone() },
                        transform: t,
                        ..default()
                    },
                    Target {
                        id,
                        perp: false,
                        outfit
                    },
                    Impulse::new(),
                    Torque::new(),
                    //RaycastMesh::<()>::default(),
                ));
                id += 1;
            }
        }
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
        let i = t.id as f32;
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
    mut title: Query<&mut Text, With<TitleText>>,
    lax: Query<(Entity, &Transform), With<LaxerFly>>,
    targets: Query<(Entity, &Transform, &Target), With<Target>>)
{
    for (le, lt) in lax.iter() {
        for (te, tt, target) in targets.iter() {
            if lt.translation.distance(tt.translation) < 4.0 {
                if let Ok(mut txt) = title.get_single_mut() {
                    let dbg = format!("...{:?}", target.outfit);
                    txt.sections[0].value = dbg.into();
                }
                cmds.entity(te).despawn_recursive();
                cmds.entity(le).despawn_recursive();
                cmds.spawn(Explosion(tt.translation));
                continue;
            }
        }
    }
}

fn got_ray_hit (
    q: Query<(&Parent, Entity, Option<&Target>), Added<RayHit>>,
    parent: Query<&Parent>

) {
    for (p, e, op) in q.iter() {
        println!("yup {:?} - {:?}", e, p);
        for ancestor in parent.iter_ancestors(e) {
            info!("{:?} is an ancestor of {:?}", ancestor, e);
        }

        if let Some(t) = op {
            println!("hit {:?}", t.id);
        }
    }
}
