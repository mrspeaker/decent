mod camera;
mod player;
mod physics;
mod laxer;
mod particles;
mod target;

use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use physics::PhysicsPlugin;
use particles::ParticlePlugin;
use laxer::LaxerPlugin;
use target::TargetPlugin;

use bevy::window::{CursorGrabMode, PrimaryWindow};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            PlayerPlugin,
            LaxerPlugin,
            TargetPlugin,
            PhysicsPlugin,
            ParticlePlugin
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
        scene: assets.load("moutain2.glb#Scene0"),
        transform: Transform::from_xyz(0.,0.,-150.).with_scale(Vec3::ONE * 1000.0),
        ..default()
    });

     /*commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 890.0,
     });*/


    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT * 0.5,
            color: Color::rgb(1.0, 0.9, 0.8),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2000.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });


    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(3.5, 350.0)),
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

    let stone = materials.add(StandardMaterial {
        base_color: Color::hex("4b3621").unwrap(),
        perceptual_roughness: 0.8,
        ..default()
    });

    // walls
    for x in &[-20., 20.] {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(0.5, 13.0, 150.0)),
            material: stone.clone(),
            transform: Transform::from_xyz(*x, 13.0/2.0, 75.0),
            ..default()
        });
    }
    // roofs
    for y in &[-0.255, 13.0] {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(40.0, 0.5, 150.0)),
            material: stone.clone(),
            transform: Transform::from_xyz(0.0, *y, 75.0),
            ..default()
        });
    }

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

    // UI
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
                    left: Val::Percent(46.25),
                    top: Val::Percent(48.0),
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

            // Preloading particles for some reason
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(0.),
                    top: Val::Percent(0.),
                    width: Val::Percent(1.0),
                    height: Val::Percent(1.0),
                    ..default()
                },
                image: assets.load("exp1.png").into(),
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
