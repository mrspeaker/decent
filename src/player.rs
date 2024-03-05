use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::physics::{Impulse, Torque, Bob};
use crate::laxer::Laxer;
use bevy_mod_raycast::prelude::*;
use crate::game::Game;

pub struct PlayerPlugin;

const SPEED: f32 = 2.0;
const SPEED_ROLL: f32 = 10.0;
const SENS: f32 = 0.01;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player);
        app.add_systems(Update, (
            update_player,
            auto_level,
            raycast
        ));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct RayHit;

fn init_player(mut cmds: Commands) {
    cmds.spawn((
        Transform::from_xyz(35.0, 30.0, 150.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        Player,
        Impulse::new(),
        Torque::new(),
        Bob
    ));
}

fn update_player(
    mut cmds: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut q: Query<(&mut Transform, &mut Torque, &mut Impulse), With<Player>>)
{
    let dt = time.delta_seconds();

    let Ok((t, mut torque, mut impulse)) = q.get_single_mut() else {
        return;
    };
    let mut rot = Vec3::ZERO;

    // Manual roll
    if keys.pressed(KeyCode::KeyZ) { rot.z = SPEED_ROLL; }
    if keys.pressed(KeyCode::KeyC) { rot.z = -SPEED_ROLL; }

    for event in mouse_events.read() {
        rot.x = -event.delta.y; // Pitch
        rot.y = -event.delta.x; // Yaw
    }

    if rot.length() > 0.0 {
        torque.add_force(rot * SENS * dt);
    }

    let mut imp = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) { imp += Vec3::from(t.forward()); }
    if keys.pressed(KeyCode::KeyS) { imp += Vec3::from(t.back()); }
    if keys.pressed(KeyCode::KeyA) { imp += Vec3::from(t.left()); }
    if keys.pressed(KeyCode::KeyD) { imp += Vec3::from(t.right()); }
    if keys.pressed(KeyCode::KeyQ) { imp += Vec3::from(t.down());}
    if keys.pressed(KeyCode::KeyE) { imp += Vec3::from(t.up()); }

    if imp.length() > 0.0 {
        impulse.add_force(imp.normalize() * SPEED * dt);
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        cmds.spawn(Laxer::new(t.translation + t.forward() * 2.0, t.rotation));
    }
}

fn auto_level(
    time: Res<Time>,
    mut q: Query<&mut Transform, With<Player>>
) {
    let Ok(mut t) = q.get_single_mut() else {
        return;
    };

    let mut roll_free = Transform::from_rotation(t.rotation);
    roll_free.look_to(Vec3::from(t.forward()), Vec3::Y);

    t.rotation = t.rotation.slerp(
        roll_free.rotation,
        time.delta_seconds() * 0.8);
}

fn raycast(
    mut cmds: Commands,
    mut raycast: Raycast,
    mut q: Query<(&Transform, &mut Impulse), With<Player>>,
    parent: Query<&Parent>,
    mut game: ResMut<Game>
) {
    let Ok((t, mut i)) = q.get_single_mut() else { return; };
    let ray = Ray3d::new(
        t.translation,
        t.rotation * -Vec3::Z
    );
    //let hits = raycast.debug_cast_ray(ray, &RaycastSettings::default(), &mut gizmos);
    let settings = RaycastSettings::default();
    let hits = raycast.cast_ray(ray, &settings);
    let Some((entity, hit)) = hits.first() else { return; };

    let dist = hit.distance();
    let min = 5.0;
    if dist < min {
        let b = ((min - dist) / min) * 0.3;
        // Bounce back
        i.add_force(
            t.back().normalize() * b +
                hit.normal().normalize() * b
        );
    }

    if dist < 50.0 {
        // Get root of mesh and attach RayHit
        let root = match parent.iter_ancestors(*entity).last() {
            Some(root) => root,
            None => *entity
        };
        cmds.entity(root).insert(RayHit);
    } else {
        // no hit
        // TODO: should be event
        game.scanning.active = false;

    }

}
