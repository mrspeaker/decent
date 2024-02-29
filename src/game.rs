use bevy::prelude::*;
use self::Adornment::*;

pub struct GamePlugin;

#[derive(Resource, Default)]
pub struct Game {
    pub score: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adornment {
    FunnyHat,
    Sunnies,
    ExtraLimb,
    Umbrella,
    RedScarf,
    FakeBeard,
    NoShirt,
    FlipFlops
}

impl Adornment {
    pub fn iter() -> impl Iterator<Item = Adornment> {
        [FunnyHat, Sunnies, ExtraLimb, Umbrella,
         RedScarf, FakeBeard, NoShirt, FlipFlops].iter().copied()
    }
}

#[derive(Event)]
pub struct GameHitEvent(pub u32);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Game>()
            .add_systems(Startup, (
                setup_game,
            ))
            .add_systems(Update, (
                game_event_system,
            ))
            .add_event::<GameHitEvent>();
    }
}

fn setup_game(
    mut game: ResMut<Game>
) {
    game.score = 1;
}

fn game_event_system(
    mut game: ResMut<Game>,
    mut ev_hit: EventReader<GameHitEvent>,
) {
    for hit in ev_hit.read() {
        game.score += hit.0 as i32;
    }
}
