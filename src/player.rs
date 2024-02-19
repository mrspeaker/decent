use bevy::prelude::*;
use bevy::math::vec3;
use crate::physics::{Velocity, Acceleration, Impulse};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player);
        app.add_systems(Update, update_player);
    }
}

#[derive(Component)]
pub struct Player;


fn init_player(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmds.spawn((
        /*PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid {half_size: Vec3::new(0.1, 0.1, 0.1)})),
            material: materials.add(Color::rgb_u8(255, 255, 255)),
            transform: Transform::from_xyz(0.0, 4.5, -9.0),
            ..default()
    },*/
        Transform::from_xyz(0.0, 4.5, -9.0),
        Player,
        Velocity(Vec3::ZERO),
        Acceleration(Vec3::ZERO)
    ));
}

fn update_player(
    mut cmds: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    q: Query<Entity, With<Player>>)
{
    let mut zo = 0.0;
    let mut xo = 0.0;
    let mut yo = 0.0;

    if keys.pressed(KeyCode::KeyW) { zo = 1.0; }
    if keys.pressed(KeyCode::KeyS) { zo = -1.0; }
    if keys.pressed(KeyCode::KeyA) { xo = 1.0; }
    if keys.pressed(KeyCode::KeyD) { xo = -1.0; }
    if keys.pressed(KeyCode::KeyQ) { yo = -1.0; }
    if keys.pressed(KeyCode::KeyE) { yo = 1.0; }

    let speed = 0.1;

    if xo != 0.0 || zo != 0.0 || yo != 0. {
        if let Ok(entity) = q.get_single() {
            cmds
                .entity(entity)
                .insert(
                    Impulse (
                        vec3(
                            xo * speed,
                            yo * speed,
                            zo * speed
                        )
                    )
                );
        }
    }
}
