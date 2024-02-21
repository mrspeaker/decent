use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::physics::{Velocity, Acceleration, Impulse, Torque, TorquePhysics};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player);
        app.add_systems(Update, update_player);
    }
}

#[derive(Component)]
pub struct Player;

fn init_player(mut cmds: Commands) {
    let mut tp = TorquePhysics::new();
    tp.spin(3.1415);

    cmds.spawn((
        Transform::from_xyz(0.0, 4.5, -9.0),
        Player,
        Velocity(Vec3::ZERO),
        Acceleration(Vec3::ZERO),
        tp
    ));
}

fn update_player(
    mut cmds: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_events: EventReader<MouseMotion>,
    mut q: Query<(Entity, &mut Transform), With<Player>>)
{
    let speed = 5.0;
    let sens = 0.01;
    let dt = time.delta_seconds();

    if let Ok((ent, t)) = q.get_single_mut() {
        let mut rot = Vec3::ZERO;

        // Manual roll
        if keys.pressed(KeyCode::KeyZ) { rot.z = speed; }
        if keys.pressed(KeyCode::KeyC) { rot.z = -speed; }

        for event in mouse_events.read() {
            rot.x = -event.delta.y; // Pitch
            rot.y = -event.delta.x; // Yaw
        }

        if rot.length() > 0.0 {
            cmds
                .entity(ent)
                .insert(Torque(rot * sens * dt));
        }

        let mut imp = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) { imp += Vec3::from(t.forward()); }
        if keys.pressed(KeyCode::KeyS) { imp += Vec3::from(t.back()); }
        if keys.pressed(KeyCode::KeyA) { imp += Vec3::from(t.left()); }
        if keys.pressed(KeyCode::KeyD) { imp += Vec3::from(t.right()); }
        if keys.pressed(KeyCode::KeyQ) { imp += Vec3::from(t.down());}
        if keys.pressed(KeyCode::KeyE) { imp += Vec3::from(t.up()); }

        if imp.length() > 0.0 {
            cmds
                .entity(ent)
                .insert(Impulse (imp * speed * dt));
        }
    }
}
