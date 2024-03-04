use bevy::prelude::*;
use self::Adornment::*;
use rand::Rng;

pub struct GamePlugin;

const ADORNMENTS_TOTAL: u32 = 8;
const ADORNMENTS_PER_PERSON: u32 = 3;
const ADORNMENTS_COMBOS: u32 = 56; // 8! / (3! x (8 - 3)!)

#[derive(Clone)]
pub struct Guess {
    pub result: u8,
    pub outfit: [Adornment; 3]
}

#[derive(Default)]
pub struct Scan {
    pub entity: Option<Entity>,
    pub time: f32,
    pub active: bool
}

#[derive(Resource, Default)]
pub struct Game {
    pub score: i32,
    pub perp: Option<u32>,
    pub perp_outfit: Option<[Adornment; 3]>,
    pub guesses: Vec<Guess>,
    pub scanning: Scan
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
    Shopping,
    Pole,
    FakeBeard,
    Swan,
    Box
}

impl Adornment {
    pub fn iter() -> impl Iterator<Item = Adornment> {
        [FunnyHat, Sunnies, ExtraLimb, Shopping,
         Pole, FakeBeard, Swan, Box].iter().copied()
    }
}

#[derive(Event)]
pub struct GameHitEvent(pub u32);

#[derive(Event)]
pub struct GameScanEvent(pub Guess);


impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Game>()
            .init_resource::<Outfits
                             >()
            .add_systems(PreStartup, (
                setup_game,
            ))
            .add_systems(Update, (
                game_event_system,
            ))
            .add_event::<GameHitEvent>()
            .add_event::<GameScanEvent>();
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
    mut ev_scan: EventReader<GameScanEvent>,
) {
    for hit in ev_hit.read() {
        game.score += hit.0 as i32;
    }
    for scan in ev_scan.read() {
        game.guesses.push(scan.0.clone());

        for guess in game.guesses.iter() {
            if guess.result == 3 {
                info!("you found them!");
            }
        }
    }
}
