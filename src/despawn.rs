use bevy::prelude::*;

pub struct DespawnPlugin;

#[derive(Component)]
pub struct Despawn;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_systems(PreUpdate, despawn);
    }
}

fn despawn(
    mut cmds: Commands,
    q: Query<Entity, Added<Despawn>>
) {
    for e in q.iter() {
        cmds.entity(e).despawn_recursive();
    }
}
