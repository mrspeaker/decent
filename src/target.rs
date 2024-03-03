use bevy::prelude::*;
use crate::camera::TitleText;
use crate::laxer::LaxerFly;
use crate::physics::{Impulse, Torque};
use crate::particles::Explosion;
use crate::player::RayHit;
use rand::Rng;
use crate::game::{GameHitEvent, Game, Adornment, Outfits};
pub struct TargetPlugin;

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game: Res<Game>,
    outfits: Res<Outfits>
) {
    // Floaty cube things
    let mut rng = rand::thread_rng();
    let car_one = assets.load("renault_logan_2004.glb#Scene0");
    let car_two = assets.load("car3.glb#Scene0");
    //let car_two = assets.load("car.glb#Scene0");

    if let Some(o) = outfits.outfits {
        for i in 0..o.len() {
            let mut t = Transform::from_xyz(
                rng.gen::<f32>() * 300.0 - 150.0,
                rng.gen::<f32>() * 100.0,
                rng.gen::<f32>() * 300.0 - 150.0
            );
            t.rotate_y(i as f32 * 37.0);

            let outfit = o[i];

            commands.spawn((
                SceneBundle {
                    scene: if i < 25 { car_one.clone() } else { car_two.clone() },
                    transform: t,
                    ..default()
                },
                Target {
                    id: i as u32,
                    perp: i as u32 == game.perp.unwrap(),
                    outfit
                },
                Impulse::new(),
                Torque::new(),
            )).with_children(|parent| {
                for (j, ad) in outfit.iter().enumerate() {
                    let c = match ad {
                        Adornment::FunnyHat => Color::rgb(0.0, 0.0, 1.0),
                        Adornment::Sunnies => Color::rgb(0.0, 1.0, 0.0),
                        Adornment::ExtraLimb => Color::rgb(1.0, 0.0, 0.0),
                        Adornment::Umbrella => Color::rgb(1.0, 0.0, 1.0),
                        Adornment::RedScarf => Color::rgb(1.0, 1.0, 0.0),
                        Adornment::FakeBeard => Color::rgb(0.0, 1.0, 1.0),
                        Adornment::Swan => Color::rgb(1.0, 1.0, 1.0),
                        Adornment::FlipFlops => Color::rgb(0.0, 0.0, 0.0)
                    };
                    if *ad == Adornment::Sunnies {
                        parent.spawn(SceneBundle {
                            scene: assets.load("glasses.glb#Scene0"),
                            transform: Transform::from_xyz(0., 2.0,3.).with_scale(Vec3::ONE * 3.0),
                            ..default()
                        });

                    } else if *ad == Adornment::ExtraLimb {
                        parent.spawn(SceneBundle {
                            scene: assets.load("arm.glb#Scene0"),
                            transform: Transform::from_xyz(1.5, 1.5,0.).with_scale(Vec3::ONE * 1.5),
                            ..default()
                        });
                    } else if *ad == Adornment::FunnyHat {
                        parent.spawn(SceneBundle {
                            scene: assets.load("hat.glb#Scene0"),
                            transform: Transform::from_xyz(0., 2.8,0.).with_scale(Vec3::ONE * 1.5),
                            ..default()
                        });
                    } else if *ad == Adornment::FakeBeard {
                        parent.spawn(SceneBundle {
                            scene: assets.load("beard.glb#Scene0"),
                            transform: Transform::from_xyz(0., 0.5,3.7).with_scale(Vec3::ONE * 4.2),
                            ..default()
                        });
                    } else if *ad == Adornment::Swan {
                        parent.spawn(SceneBundle {
                            scene: assets.load("swan.glb#Scene0"),
                            transform: Transform::from_xyz(0., 2.8,-1.5).with_scale(Vec3::ONE * 2.0),
                            ..default()
                        });
                    } else {
                        parent.spawn(
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 0.5, 0.5))),
                                material: materials.add(c),
                                transform: Transform::from_xyz(j as f32 - 1.0, 3.0, 0.0),
                                ..default()
                            });
                    }
                }
            });
        }
    }
}

fn move_targets (
    time: Res<Time>,
    mut q: Query<(
        &mut Transform,
        &Target,
        &mut Impulse,
        &mut Torque)>)
{
    let dt = time.delta_seconds();
    let elapsed = time.elapsed_seconds();

    let mut rng = rand::thread_rng();

    for (mut tr, t, mut imp, mut tor) in q.iter_mut() {
        let up = tr.up();
        let i = t.id as f32;
        tr.translation += up * dt *
                (elapsed * (2.0 + i) * 1.).sin() *
            ((i - 50.0) * 0.1);

        if imp.speed() < 0.01 {
            if rng.gen::<f32>() * 100.0 < 0.5 {
                imp.add_force(Vec3::new(
                    rng.gen::<f32>() - 0.5,
                    rng.gen::<f32>() - 0.5,
                    rng.gen::<f32>() - 0.5
                ).normalize() * 0.4);
            }
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
    targets: Query<(Entity, &Transform, &Target), With<Target>>,
    mut ev_hit: EventWriter<GameHitEvent> )
{
    for (le, lt) in lax.iter() {
        for (te, tt, trg) in targets.iter() {
            if lt.translation.distance(tt.translation) < 4.0 {
                cmds.entity(te).despawn_recursive();
                cmds.entity(le).despawn_recursive();
                cmds.spawn(Explosion(tt.translation));
                ev_hit.send(GameHitEvent(2));
                if trg.perp {
                    cmds.spawn(Explosion(tt.translation + Vec3::Y * 5.0));
                    cmds.spawn(Explosion(tt.translation + Vec3::X * 5.0));
                    cmds.spawn(Explosion(tt.translation + Vec3::Z * 5.0));
                }
                continue;
            }
        }
    }
}

fn got_ray_hit (
    mut cmds: Commands,
    q: Query<(Entity, &Target), Added<RayHit>>,
    mut title: Query<&mut Text, With<TitleText>>,
    game: Res<Game>
) {
    for (e, t) in q.iter() {
        let Ok(mut txt) = title.get_single_mut() else { continue; };
        let perp_outfit = game.perp_outfit.unwrap();
        let dbg = format!("...{:?} {:?}", game.score, t.outfit.map(|o| {
            let has = perp_outfit.iter().any(|&x| x == o);
            return (has, o);
        }));
        txt.sections[0].value = dbg.into();

        // TODO: inefficient: will add/remove constantly while scanning.
        cmds.entity(e).remove::<RayHit>();
    }
}
