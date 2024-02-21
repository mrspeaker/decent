use bevy::prelude::*;

pub struct LaxerPlugin;

#[derive(Component)]
pub struct LaxerFly;

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
            .add_systems(Update, add_new_laxers)
            .add_systems(Update, update_laxers);
    }
}

fn add_new_laxers(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<(Entity, &Laxer), Added<Laxer>>) {
    for (_, l) in q.iter() {
        info!("added. {:?}", l.pos);
        cmds.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid {half_size: Vec3::new(0.1, 0.1, 0.5)})),
                material: materials.add(Color::rgb_u8(255, 255, 0)),
                transform: Transform::from_translation(l.pos).with_rotation(l.dir),
                ..default()
            },
            LaxerFly
        ));
    }
}

fn update_laxers (
    mut q: Query<&mut Transform, With<LaxerFly>>) {
    for mut t in q.iter_mut() {
        let fwd = t.forward();
        t.translation += fwd * 1.0;
    }
}