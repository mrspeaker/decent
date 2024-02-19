mod camera;
mod player;
mod physics;

use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use physics::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin { is3d: true })
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Sprite;

fn startup(mut cmds: Commands, ass: Res<AssetServer>) {
    for j in 0..10 {
        for i in 0..10 {
            cmds.spawn((
                SpriteBundle {
                    texture: ass.load("star.png"),
                    transform: Transform {
                        translation: Vec3::new(
                            -200.0 + (i as f32) * 50.0,
                            -200.0 + (j as f32) * 50.0,
                            0.0),
                        scale: Vec3::splat(0.2),
                        ..default()
                    },
                    ..default()
                },
                Sprite
            ));
        }
    }
}



fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d { normal: Direction3d::Y })),
        material: materials.add(Color::rgb(1., 0.9, 0.9)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.0)),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid {half_size: Vec3::new(1.0, 0.1, 0.3)})),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
