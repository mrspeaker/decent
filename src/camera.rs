use bevy::prelude::*;
use crate::player::Player;

pub struct CameraPlugin {
    pub is3d: bool
}

#[derive(Component)]
pub struct Camera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        if !self.is3d {
            app.add_systems(Startup, init_camera);
        } else {
            app
                .add_systems(Startup, init_camera3d)
                .add_systems(Update, sync_camera);
        }
    }
}

fn init_camera(mut cmds: Commands) {
    cmds.spawn((Camera2dBundle::default(),Camera));
}

fn init_camera3d(mut cmds: Commands) {
    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(
                0.0,
                0.0,
                -9.0
            ).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
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
        }
    }
}
