use bevy::prelude::*;
use crate::camera::TitleText;
use crate::laxer::LaxerFly;
use crate::physics::{Impulse, Torque};
use crate::particles::Explosion;
use crate::player::RayHit;
use rand::Rng;
use crate::game::{GameHitEvent, GameScanEvent, Guess, Game, Adornment, Outfits};
use crate::despawn::Despawn;
use std::f32::consts::PI;

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
    game: Res<Game>,
    outfits: Res<Outfits>
) {
    let mut rng = rand::thread_rng();
    let car_two = assets.load("car3.glb#Scene0");
    let car_one = assets.load("car.glb#Scene0");

    let Some(o) = outfits.outfits else { return; };
    for i in 0..o.len() {
        let mut t = Transform::from_xyz(
            rng.gen::<f32>() * 150.0 - 75.0,
            rng.gen::<f32>() * 40.0 + 20.0,
            rng.gen::<f32>() * 150.0 - 75.0 + 50.0
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
            for ad in outfit.iter() {
                if *ad == Adornment::Sunnies {
                    parent.spawn(SceneBundle {
                        scene: assets.load("glasses.glb#Scene0"),
                        transform: Transform::from_xyz(0., 0.9,1.8),
                        ..default()
                    });

                } else if *ad == Adornment::ExtraLimb {
                    parent.spawn(SceneBundle {
                        scene: assets.load("arm.glb#Scene0"),
                        transform: Transform::from_xyz(0.5, 0.75, 0.25),
                        ..default()
                    });
                } else if *ad == Adornment::FunnyHat {
                    parent.spawn(SceneBundle {
                        scene: assets.load("hat.glb#Scene0"),
                        transform: Transform::from_xyz(0., 1.5,0.),
                        ..default()
                    });
                } else if *ad == Adornment::FakeBeard {
                    parent.spawn(SceneBundle {
                        scene: assets.load("beard.glb#Scene0"),
                        transform: Transform::from_xyz(0., 0.5,1.8),
                        ..default()
                    });
                } else if *ad == Adornment::Swan {
                    parent.spawn(SceneBundle {
                        scene: assets.load("swan.glb#Scene0"),
                        transform: Transform::from_xyz(0., 1.5,-0.5),
                        ..default()
                    });
                } else if *ad == Adornment::Pole {
                    parent.spawn(SceneBundle {
                        scene: assets.load("pole.glb#Scene0"),
                        transform: Transform::from_xyz(0.0, 1.2,-1.4)
                            .with_rotation(Quat::from_rotation_x(-PI / 4.0)),
                        ..default()
                    });
                } else if *ad == Adornment::Box {
                    parent.spawn(SceneBundle {
                        scene: assets.load("box.glb#Scene0"),
                        transform: Transform::from_xyz(0.0, 0.9,1.4)
                            .with_rotation(Quat::from_rotation_x(0.4)),
                        ..default()
                    });
                } else if *ad == Adornment::Shopping {
                    parent.spawn(SceneBundle {
                        scene: assets.load("shopping.glb#Scene0"),
                        transform: Transform::from_xyz(-1.2, 0.4,0.0)
                            .with_rotation(Quat::from_rotation_x(1.0)),
                        ..default()
                    });
                }
            }
        });
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

    let torque_power = 0.05;
    let impulse_power = 0.3;

    for (mut tr, t, mut imp, mut tor) in q.iter_mut() {
        let up = tr.up();
        let i = t.id as f32;
        tr.translation += up * dt *
                (elapsed * (2.0 + i) * 1.).sin() *
            ((i - 50.0) * 0.02);

        if imp.speed() < 0.01 {
            if rng.gen::<f32>() * 1000.0 < 0.5 {
                imp.add_force(Vec3::new(
                    rng.gen::<f32>() - 0.5,
                    rng.gen::<f32>() - 0.5,
                    rng.gen::<f32>() - 0.5
                ).normalize() * impulse_power);
            }
        }

        if rng.gen::<f32>() * 1000.0 < 0.5 {
            tor.add_force(Vec3::new(
                0.,
                rng.gen::<f32>() - 0.5,
                0.
                    ).normalize() * torque_power);
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
                //cmds.entity(te).despawn_recursive();
                cmds.entity(te).insert(Despawn);
                cmds.entity(le).insert(Despawn);
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
    time: Res<Time>,
    q: Query<(Entity, &Target), Added<RayHit>>,
    mut title: Query<&mut Text, With<TitleText>>,
    mut game: ResMut<Game>,
    mut ev_scan: EventWriter<GameScanEvent>
) {
    let Some(perp_outfit) = game.perp_outfit else { return; };
    let Ok(mut txt) = title.get_single_mut() else { return; };

    for (e, t) in q.iter() {
        txt.sections[0].value = "scanning////".into();

        if game.scanning.entity != Some(e)  {
            game.scanning.time = 0.0;
            game.scanning.entity = Some(e);
        } else {
            let last = game.scanning.time;
            game.scanning.time += time.delta_seconds();
            if game.scanning.time > 2.0 {
                let count = t.outfit.iter()
                    .map(|o| perp_outfit.iter().any(|&x| x == *o))
                    .filter(|o| *o)
                    .count();
                let dbg = format!("   matched: {:?}.", count);
                txt.sections[0].value = dbg.into();
                    // Only send once.
                if last <= 2.0 {
                    ev_scan.send(
                        GameScanEvent(
                            Guess {
                                result: count as u8,
                                outfit: t.outfit
                            }
                        ));
                }
            }
        }

        // TODO: inefficient: will add/remove constantly while scanning.
        cmds.entity(e).remove::<RayHit>();
    }
}
