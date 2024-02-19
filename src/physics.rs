use bevy::prelude::*;
use bevy::math::vec3;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_impulses);
        app.add_systems(Update, integrate);
    }
}


#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Debug)]
pub struct Impulse(pub Vec3);

fn apply_impulses(
    mut cmds: Commands,
    mut q: Query<(Entity, &Impulse, &mut Acceleration)>) {

    for (ent, imp, mut acc) in q.iter_mut() {
        acc.0 = acc.0 + imp.0;
        cmds.entity(ent).remove::<Impulse>();
    }
}


fn integrate(mut q:Query<(&mut Transform, &mut Velocity, &mut Acceleration)>) {
    for (mut t, mut vel, mut acc) in q.iter_mut() {
        vel.0 = vel.0 + acc.0;
        vel.0 = vel.0 * 0.9; // Lol, friction
        //info!("v: {:?}", vel.0.y);
        acc.0 = Vec3::ZERO;

        t.translation += vel.0;
    }
}
