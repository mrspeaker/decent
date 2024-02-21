mod camera;
mod player;
mod physics;

use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use physics::PhysicsPlugin;
use rand::Rng;

use bevy::window::{CursorGrabMode, PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, cursor_grab)
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 250.0)),
        material: materials.add(Color::rgb(1., 0.9, 0.9)),
        transform: Transform::from_translation(Vec3::new(0., -10., 10.0)),
        ..Default::default()
    });

    let mut rng = rand::thread_rng();
    for _ in 1..100 {
        let x: f32 = rng.gen();
        let xx = x * 30.0 -15.0;
        let y: f32 = rng.gen();
        let yy = y * 8.0;
        let z: f32 = rng.gen();
        let zz = z * 200.;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid {half_size: Vec3::new(1.0, 0.1, 0.3)})),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(xx, yy, zz),
            ..default()
        });
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

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

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Decent",
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            ));
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(45.0),
                    top: Val::Percent(50.0),
                    width: Val::Percent(7.5),
                    ..default()
                },
                image: assets.load("sight.png").into(),
                ..default()
            });
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(125.0),
                    ..default()
                },
                image: assets.load("cockpit.png").into(),
                ..default()
            });
        });


}

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

fn cursor_ungrab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::None;
    primary_window.cursor.visible = true;
}
