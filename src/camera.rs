use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use crate::player::Player;

pub struct CameraPlugin;

#[derive(Component)]
pub struct Camera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AtmospherePlugin)
            .add_systems(Startup, init_camera)
            .add_systems(Update, sync_camera);
    }
}

fn init_camera(mut cmds: Commands) {
    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(
                0.0,
                0.0,
                0.0
            ).looking_to(Vec3::Z, Vec3::Y),
            ..default()
        },
        AtmosphereCamera::default(),
        Camera
    ));
}

fn sync_camera(
    player: Query<&Transform, With<Player>>,
    mut q: Query<&mut Transform, (With<Camera>, Without<Player>)>
) {
    if let Ok(p) = player.get_single() {
        if let Ok(mut t) = q.get_single_mut() {
            t.translation = p.translation;
            t.rotation = p.rotation;
        }
    }
}
