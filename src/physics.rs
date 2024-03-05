use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            bob,
            integrate
        ));
    }
}

#[derive(Component)]
pub struct Bob;

#[derive(Component)]
pub struct Impulse {
    vel: Vec3,
    acc: Vec3
}
impl Impulse {
    pub fn new() -> Self {
        Self {
            vel: Vec3::ZERO,
            acc: Vec3::ZERO
        }
    }

    pub fn add_force(&mut self, v: Vec3) {
        self.acc += v;
    }

    pub fn speed(&self) -> f32 {
        return self.vel.length();
    }
}

#[derive(Component)]
pub struct Torque {
    vel: Vec3,
    acc: Vec3
}

impl Torque {
    pub fn new() -> Self {
        Self {
            vel: Vec3::ZERO,
            acc: Vec3::ZERO
        }
    }

    pub fn add_force(&mut self, v: Vec3) {
        self.acc += v;
    }
}

fn integrate(mut q:Query<(&mut Transform, &mut Impulse, &mut Torque)>) {
    for (mut t, mut impulse, mut torque) in q.iter_mut() {

        // Translation
        let acc = impulse.acc;
        impulse.vel += acc;
        impulse.vel *= 0.93; // Lol, friction
        impulse.acc = Vec3::ZERO;

        t.translation += impulse.vel;

        // Rotation
        let arot = torque.acc;
        torque.vel += arot;
        torque.vel *= 0.90; // Lol, friction
        torque.acc = Vec3::ZERO;

        let a = (*t.forward()).dot(Vec3::Y).abs();
        t.rotate_local_x(torque.vel.x);
        if a < 0.4 {
            t.rotate_y(torque.vel.y); // Global y to eliminate roll
        } else {
            t.rotate_local_y(torque.vel.y);
        }

        t.rotate_local_z(torque.vel.z);
    }
}

fn bob(
    time: Res<Time>,
    mut q:Query<(&Transform, &mut Impulse), With<Bob>>
) {
    for (t, mut impulse) in q.iter_mut() {
        let up = t.up();
        impulse.acc += up * 0.001 * (time.elapsed_seconds() * 3.0).sin();
    }
}
