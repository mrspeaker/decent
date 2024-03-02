use bevy::prelude::*;
use self::Adornment::*;
use rand::Rng;

pub struct GamePlugin;

const ADORNMENTS_TOTAL: u32 = 8;
const ADORNMENTS_PER_PERSON: u32 = 3;
const ADORNMENTS_COMBOS: u32 = 56; // 8! / (3! x (8 - 3)!)

#[derive(Resource, Default)]
pub struct Game {
    pub score: i32,
    pub perp: Option<u32>,
    pub perp_outfit: Option<[Adornment; 3]>
}

#[derive(Resource, Default)]
pub struct Outfits {
    pub outfits: Option<[[Adornment; ADORNMENTS_PER_PERSON as usize]; ADORNMENTS_COMBOS as usize]>
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
            .init_resource::<Outfits>()
            .add_systems(PreStartup, (
                setup_game,
            ))
            .add_systems(Update, (
                game_event_system,
            ))
            .add_event::<GameHitEvent>();
    }
}

fn setup_game(
    mut game: ResMut<Game>,
    mut outfits: ResMut<Outfits>
) {
    let mut rng = rand::thread_rng();

    game.score = 1;
    let perp_id = rng.gen_range(0..ADORNMENTS_COMBOS + 1);
    game.perp = Some(perp_id);

    let ads: Vec<_> = Adornment::iter().collect();
    let mut o: Vec<[Adornment; 3]> = Vec::new();

    for i in 0..ADORNMENTS_TOTAL {
        for j in i+1..ADORNMENTS_TOTAL {
            for k in j+1..ADORNMENTS_TOTAL {
                let outfit = [
                    ads[i as usize],
                    ads[j as usize],
                    ads[k as usize]
                ];
                if o.len() == perp_id as usize {
                    game.perp_outfit = Some(outfit);
                }
                o.push(outfit);
            }
        }
    }
    outfits.outfits = o[..].try_into().ok();
}

fn game_event_system(
    mut game: ResMut<Game>,
    mut ev_hit: EventReader<GameHitEvent>,
) {
    for hit in ev_hit.read() {
        game.score += hit.0 as i32;
    }
}
