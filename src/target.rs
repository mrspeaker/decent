use bevy::prelude::*;
use crate::camera::TitleText;
use crate::laxer::LaxerFly;
use crate::physics::{Impulse, Torque};
use crate::particles::Explosion;
use crate::player::RayHit;
use rand::Rng;
use crate::game::{GameHitEvent, Game, Adornment};
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
                        perp: id == 0,
                        outfit
                    },
                    Impulse::new(),
                    Torque::new(),
                )).with_children(|parent| {
                    for (i, o) in outfit.iter().enumerate() {
                        let c = match o {
                            Adornment::FunnyHat => Color::rgb(0.0, 0.0, 1.0),
                            Adornment::Sunnies => Color::rgb(0.0, 1.0, 0.0),
                            Adornment::RedScarf => Color::rgb(1.0, 0.0, 0.0),
                            Adornment::Umbrella => Color::rgb(1.0, 0.0, 1.0),
                            Adornment::ExtraLimb => Color::rgb(1.0, 1.0, 0.0),
                            Adornment::FakeBeard => Color::rgb(1.0, 0.0, 1.0),
                            Adornment::NoShirt => Color::rgb(0.0, 1.0, 1.0),
                            _ => Color::rgb(1.0, 1.0, 1.0)
                        };

                        parent.spawn(
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 0.5, 0.5))),
                                material: materials.add(c),
                                transform: Transform::from_xyz(i as f32 - 1.0, 3.0, 0.0),
                                ..default()
                            });
                    }
                });
                id += 1;
            }
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
        if let Ok(mut txt) = title.get_single_mut() {
            let dbg = format!("...{:?} {:?}", game.score, t.outfit);
            txt.sections[0].value = dbg.into();
        }

        // TODO: inefficient: will add/remove constantly while scanning.
        cmds.entity(e).remove::<RayHit>();
    }
}
