use bevy::prelude::*;

pub struct GamePlugin;

#[derive(Component)]
pub struct GameEvent(pub u32);

#[derive(Resource, Default)]
pub struct Game {
    pub score: i32,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Game>()
            .add_systems(Startup, (
                setup_game,
            ))
            .add_systems(Update, (
                game_event_system,
            ));
    }
}

fn setup_game(
    mut game: ResMut<Game>
) {
    game.score = 0;
}

fn game_event_system(
    mut game: ResMut<Game>,
    q: Query<&GameEvent, Added<GameEvent>>
) {
    for e in q.iter() {
        game.score +=1 ;
    }
}
