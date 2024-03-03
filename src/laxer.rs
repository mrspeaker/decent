use bevy::prelude::*;
use crate::despawn::Despawn;

pub struct LaxerPlugin;

#[derive(Component)]
pub struct LaxerFly {
    time: f32
}

impl LaxerFly {
    pub fn new() -> Self {
        Self {
            time: 2.0
        }
    }
}

#[derive(Component)]
pub struct Laxer {
    pos: Vec3,
    dir: Quat
}

impl Laxer {
    pub fn new(t: Vec3, q: Quat) -> Self {
        Self {
            pos: t,
            dir: q
        }
    }
}

impl Plugin for LaxerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                add_new_laxers,
                update_laxers
            ));
    }
}

fn add_new_laxers(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<(Entity, &Laxer), Added<Laxer>>) {
    for (_, l) in q.iter() {
        let t = Transform::from_translation(l.pos).with_rotation(l.dir);
        cmds.spawn(
            (
                LaxerFly::new(),
                SpatialBundle {
                    transform: t,
                    ..default()
                }
            ))
            .with_children(|parent| {
                for x in &[-1.0, 1.0] {
                    parent.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.2, 0.2, 8.0))),
                        material: materials.add(StandardMaterial {
                            base_color: Color::hex("ff00ff").unwrap(),
                            unlit: true,
                            ..default()
                        }),
                        transform: Transform::default().with_translation(t.right() * *x),
                        ..default()
                    });
                }
            });
    }
}

fn update_laxers (
    mut cmds: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &mut Transform, &mut LaxerFly)>) {
    for (e, mut t, mut l) in q.iter_mut() {
        let fwd = t.forward();
        t.translation += fwd * 400.0 * time.delta_seconds();
        l.time -= time.delta_seconds();
        if l.time <= 0.0 {
            cmds.entity(e).insert(Despawn);
        }
    }
}
