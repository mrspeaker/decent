use bevy::prelude::*;
use std::f32::consts::PI;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_scene);
    }
}

fn init_scene(
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

}
