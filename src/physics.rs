use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, integrate);
    }
}


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

fn integrate(time: Res<Time>, mut q:Query<(&mut Transform, &mut Impulse, &mut Torque)>) {
    for (mut t, mut impulse, mut torque) in q.iter_mut() {
        // Translation
        let acc = impulse.acc;
        impulse.vel += acc;
        impulse.vel *= 0.95; // Lol, friction
        impulse.acc = Vec3::ZERO;

        // Bob.
        let up = t.up();
        t.translation += impulse.vel + (up * 0.005 * (time.elapsed_seconds() * 3.0).sin());
        if t.translation.y < 1.2 {
            t.translation.y = 1.2;
        }

        // Rotation
        let arot = torque.acc;
        torque.vel += arot;
        torque.vel *= 0.90; // Lol, friction
        torque.acc = Vec3::ZERO;

        t.rotate_local_x(torque.vel.x);
        t.rotate_local_y(torque.vel.y);
        t.rotate_local_z(torque.vel.z);
    }
}
