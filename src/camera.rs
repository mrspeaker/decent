use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use crate::player::Player;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::render::view::ColorGrading;
use bevy::render::camera::Exposure;

#[derive(Component)]
pub struct Camera;

pub struct CameraPlugin;

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
            camera: bevy::prelude::Camera {
                hdr: true,
                ..default()
            },
            projection: PerspectiveProjection { far: 1000.0, ..default() }.into(),
            //tonemapping: Tonemapping::ReinhardLuminance, //Tonemapping::TonyMcMapface,
            exposure: Exposure::BLENDER,
            color_grading: ColorGrading {
                exposure: 0.0,
                gamma: 1.8,
                pre_saturation: 1.5,
                post_saturation: 1.0,
            },
            transform: Transform::from_xyz(
                0.0,
                0.0,
                0.0
            ).looking_to(Vec3::Z, Vec3::Y),
            ..default()
        },
        FogSettings {
            color: Color::rgba(0.15, 0.15, 0.15, 1.0),
            falloff: FogFalloff::Linear {
                start: 150.0,
                end: 500.0,
            },
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
    let Ok(p) = player.get_single() else { return; };
    let Ok(mut t) = q.get_single_mut() else { return; };

    t.translation = p.translation;
    t.rotation = p.rotation;
}
