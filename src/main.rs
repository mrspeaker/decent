mod camera;
mod player;
mod physics;
mod laxer;
mod particles;
mod target;
mod game;
mod despawn;
mod ui;

use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use physics::PhysicsPlugin;
use particles::ParticlePlugin;
use laxer::LaxerPlugin;
use target::TargetPlugin;
use game::GamePlugin;
use despawn::DespawnPlugin;
use ui::UIPlugin;

use bevy::window::{CursorGrabMode, PrimaryWindow};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DespawnPlugin,
            CameraPlugin,
            GamePlugin,
            PlayerPlugin,
            LaxerPlugin,
            TargetPlugin,
            PhysicsPlugin,
            ParticlePlugin,
            UIPlugin,
        ))
        .add_systems(Startup, (setup, cursor_grab))
        .add_systems(Update, (cursor_ungrab, draw_gizmos))
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Terrain
    commands.spawn(SceneBundle {
        scene: assets.load("mountain.glb#Scene0"),
        transform: Transform::from_xyz(0.,0.,0.).with_scale(Vec3::ONE * 1.0),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: assets.load("town.glb#Scene0"),
        transform: Transform::from_xyz(0.,0.,0.).with_scale(Vec3::ONE * 1.0),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: assets.load("Michelle.glb#Scene0"),
        transform: Transform::from_xyz(0.,0.,0.).with_scale(Vec3::ONE * 1.0),
        ..default()
    });

     commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
     });

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            //color: Color::rgb(1.0, 1.0, 1.0),
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 1.0, 0.0),
            rotation: Quat::from_rotation_x(-PI * 0.5),
            ..default()
        },
        ..default()
    });

    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(3.5, 3.5)),
        material: materials.add(Color::rgb(1., 0.9, 0.8)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.0)),
        ..Default::default()
    });

    let h = 1.75;
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(0.5, h, 0.4))),
        material: materials.add(Color::rgb_u8(255, 244, 255)),
        transform: Transform::from_xyz(0., h / 2.0, -0.5),
        ..default()
    });

    // point light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });

    // text
    commands.spawn(
        TextBundle::from_section(
            "Testaroo",
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        })
    );

}

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

fn cursor_ungrab(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        let mut primary_window = q_windows.single_mut();
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    }
}

fn draw_gizmos(
    mut gizmos: Gizmos,
) {
    gizmos.arrow(Vec3::ZERO, Vec3::Y * 1.0, Color::BLUE);
    gizmos.arrow(Vec3::ZERO, Vec3::X * 1.0, Color::RED);
    gizmos.arrow(Vec3::ZERO, Vec3::Z * 1.0, Color::GREEN);
    gizmos.cuboid(
        Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(1.0)),
        Color::BLACK,
    );

}
