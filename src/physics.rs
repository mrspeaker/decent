use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            apply_impulses,
            apply_torque,
            integrate
        ));
    }
}


#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Debug)]
pub struct Impulse(pub Vec3);

#[derive(Component)]
pub struct Torque(pub Vec3);

#[derive(Component)]
pub struct TorquePhysics {
    vel: Vec3,
    acc: Vec3
}

impl TorquePhysics {
    pub fn new() -> Self {
        Self {
            vel: Vec3::ZERO,
            acc: Vec3::ZERO
        }
    }

    pub fn spin(&mut self, v: f32) {
        self.vel.y = v;
    }
}

fn apply_impulses(
    mut cmds: Commands,
    mut q: Query<(Entity, &Impulse, &mut Acceleration)>) {

    for (ent, imp, mut acc) in q.iter_mut() {
        acc.0 = acc.0 + imp.0;
        cmds.entity(ent).remove::<Impulse>();
    }
}

fn apply_torque(
    mut cmds: Commands,
    mut q: Query<(Entity, &Torque, &mut TorquePhysics)>) {

    for (ent, torque, mut tphys) in q.iter_mut() {
        tphys.acc += torque.0;
        cmds.entity(ent).remove::<Torque>();
    }
}


fn integrate(time: Res<Time>, mut q:Query<(&mut Transform, &mut Velocity, &mut Acceleration, &mut TorquePhysics)>) {
    for (mut t, mut vel, mut acc, mut tphys) in q.iter_mut() {
        // Translation
        vel.0 += acc.0;
        vel.0 *= 0.95; // Lol, friction
        acc.0 = Vec3::ZERO;

        // Bob.
        let up = t.up();
        t.translation += vel.0 + (up * 0.005 * (time.elapsed_seconds() * 3.0).sin());
        if t.translation.y < 2.0 {
            t.translation.y = 2.0;
        }

        // Rotation
        let arot = tphys.acc;
        tphys.vel += arot;
        tphys.vel *= 0.90; // Lol, friction
        tphys.acc = Vec3::ZERO;

        t.rotate_local_x(tphys.vel.x);
        t.rotate_local_y(tphys.vel.y);
        t.rotate_local_z(tphys.vel.z);
    }
}
