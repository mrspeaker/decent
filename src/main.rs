mod camera;
mod player;
mod physics;
mod laxer;

use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use physics::PhysicsPlugin;
use laxer::LaxerPlugin;

use rand::Rng;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::pbr::NotShadowCaster;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            PlayerPlugin,
            LaxerPlugin,
            PhysicsPlugin
        ))
        .add_systems(Startup, (setup, cursor_grab))
        .add_systems(Update, cursor_ungrab)
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn(SceneBundle {
        scene: assets.load("mountains.gltf#Scene0"),
        transform: Transform::from_xyz(0.,0.,-150.).with_scale(Vec3::ONE * 1000.0),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });


    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 350.0)),
        material: materials.add(Color::rgb(1., 0.9, 0.9)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.0)),
        ..Default::default()
    });

    let mut rng = rand::thread_rng();
    for _ in 1..100 {
        let x: f32 = rng.gen();
        let xx = x * 30.0 -15.0;
        let y: f32 = rng.gen();
        let yy = y * 10.0;
        let z: f32 = rng.gen();
        let zz = z * 200. - 100.0;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid {half_size: Vec3::new(1.0, 0.1, 0.3)})),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(xx, yy, zz),
            ..default()
        });
    }

    let stone = materials.add(StandardMaterial {
        base_color: Color::hex("68625B").unwrap(),
        perceptual_roughness: 0.8,
        ..default()
    });

    // pillars
    for (x, z) in &[(-20., 50.), (20., 50.)] {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 20.0, 150.0)),
            material: stone.clone(),
            transform: Transform::from_xyz(*x, 10., *z),
            ..default()
        });
    }
    // roofs
    for y in &[13.0, 20.0] {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(40.0, 1.0, 150.0)),
            material: stone.clone(),
            transform: Transform::from_xyz(0.0, *y, 45.0),
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

        // sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("6688cc").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(1_000.0)),
            ..default()
        },
        NotShadowCaster
    ));


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
                "...Decent",
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
    keys: Res<ButtonInput<KeyCode>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        let mut primary_window = q_windows.single_mut();
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    }
}
