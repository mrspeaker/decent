use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::physics::{Impulse, Torque};
use crate::laxer::Laxer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player);
        app.add_systems(Update, (
            update_player,
            auto_level
        ));
    }
}

#[derive(Component)]
pub struct Player;

fn init_player(mut cmds: Commands) {
    cmds.spawn((
        Transform::from_xyz(0.0, 8.0, 200.0),
        Player,
        Impulse::new(),
        Torque::new()
    ));
}

fn update_player(
    mut cmds: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut q: Query<(Entity, &mut Transform, &mut Torque, &mut Impulse), With<Player>>)
{
    let speed = 5.0;
    let sens = 0.01;
    let dt = time.delta_seconds();

    if let Ok((ent, t, mut torque, mut impulse)) = q.get_single_mut() {
        let mut rot = Vec3::ZERO;

        // Manual roll
        if keys.pressed(KeyCode::KeyZ) { rot.z = speed * 4.0; }
        if keys.pressed(KeyCode::KeyC) { rot.z = -speed * 4.0; }

        for event in mouse_events.read() {
            rot.x = -event.delta.y; // Pitch
            rot.y = -event.delta.x; // Yaw
        }

        if rot.length() > 0.0 {
            torque.add_force(rot * sens * dt);
        }

        let mut imp = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) { imp += Vec3::from(t.forward()); }
        if keys.pressed(KeyCode::KeyS) { imp += Vec3::from(t.back()); }
        if keys.pressed(KeyCode::KeyA) { imp += Vec3::from(t.left()); }
        if keys.pressed(KeyCode::KeyD) { imp += Vec3::from(t.right()); }
        if keys.pressed(KeyCode::KeyQ) { imp += Vec3::from(t.down());}
        if keys.pressed(KeyCode::KeyE) { imp += Vec3::from(t.up()); }

        if imp.length() > 0.0 {
            impulse.add_force(imp.normalize() * speed * dt);
        }

        if mouse_buttons.just_pressed(MouseButton::Left) {
            cmds.spawn(Laxer::new(t.translation + t.forward() * 2.0, t.rotation));
        }
    }
}


fn auto_level(
    mut gizmos: Gizmos,
    mut q: Query<&mut Transform, With<Player>>
) {
    if let Ok(t) = q.get_single_mut() {
        gizmos.arrow(Vec3::ZERO, t.rotation.to_axis_angle().0, Color::YELLOW);
    }
}
