use bevy::prelude::*;
use crate::laxer::LaxerFly;

pub struct TargetPlugin;

#[derive(Component)]
pub struct Target;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                check_collisions,
            ));
    }
}

fn check_collisions (
    mut cmds: Commands,
    lax: Query<(Entity, &Transform), With<LaxerFly>>,
    q: Query<(Entity, &Transform), With<Target>>)
{
    for (le, lt) in lax.iter() {
        for (te, tt) in q.iter() {
            if lt.translation.distance(tt.translation) < 2.5 {
                cmds.entity(te).despawn();
                cmds.entity(le).despawn_recursive();
            }
        }
    }
}
