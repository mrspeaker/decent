use bevy::prelude::*;
use crate::game::{Adornment, Game};

#[derive(Component)]
pub struct TitleText;

#[derive(Component)]
struct MatrixText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, update_scanning);
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // UI
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(46.25),
                    top: Val::Percent(48.0),
                    width: Val::Percent(7.5),
                    ..default()
                },
                image: assets.load("sight.png").into(),
                ..default()
            });
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(125.0),
                    ..default()
                },
                image: assets.load("cockpit.png").into(),
                ..default()
            });

            parent.spawn((
                TextBundle::from_section(
                    "...",
                    TextStyle {
                        font_size: 20.,
                        ..default()
                    },
                ).with_style(Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(43.0),
                    left: Val::Percent(49.0),
                    ..default()
                }),
                TitleText));

            parent.spawn((
                TextBundle::from_section(
                    "matrix",
                    TextStyle {
                        font_size: 16.,
                        ..default()
                    },
                ).with_style(Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(0.0),
                    right: Val::Percent(50.0),
                    ..default()
                }),
                MatrixText));


            // TODO: hack to Preloading particles (for some reason!)
            // Image doesn't show on particles if you don't. Need to figure
            // out the correct way to "preload".
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(0.),
                    top: Val::Percent(0.),
                    width: Val::Percent(1.0),
                    height: Val::Percent(1.0),
                    ..default()
                },
                image: assets.load("exp1.png").into(),
                ..default()
            });

        });
}

fn update_scanning(
    time: Res<Time>,
    game: Res<Game>,
    mut q: Query<&mut Text, With<TitleText>>,
    mut matrix: Query<&mut Text, (With<MatrixText>, Without<TitleText>)>
) {
    // set text
    let Ok(mut txt) = q.get_single_mut() else { return; };
    let Ok(mut mat) = matrix.get_single_mut() else { return; };

    let s = if game.scanning.active {
        match game.scanning.entity {
            Some(_) => {
                if game.scanning.time > 2.0 {
                    format!("matched: {:?}", game.guesses.last().map(|g| g.result).unwrap_or(0))
                } else {
                    const ANIM: [char;4] = ['/', '-', '\\' ,'|'];
                    let sec = ((time.elapsed_seconds() * 5.0) % 4.0) as usize;
                    format!("scan {}", ANIM[sec])
                }
            },
            None => "".to_string()
        }
    } else {
        "".to_string()
    };
    txt.sections[0].value = s;

    // Draw matrix
    let counts = game.guesses.iter().map(|g| format!("{}", g.result)).collect::<Vec<String>>().join("   ");
    let guesstrix = Adornment::iter()
        .map(|a| {
            let gs = game.guesses.iter()
                .map(|g| if g.outfit.contains(&a) {
                    "X".to_string()
                } else {
                    " ".to_string()
                });
            let nom = match a {
                Adornment::FunnyHat => "FunnyHat",
                Adornment::Sunnies => "Sunnies",
                Adornment::ExtraLimb => "ExtraLimb",
                Adornment::Shopping => "Shopping",
                Adornment::Pole => "Pole",
                Adornment::FakeBeard => "FakeBeard",
                Adornment::Swan => "Swan",
                Adornment::Box => "Box",
            };
            format!("{:<10}: {}", nom, gs.collect::<Vec<String>>().join("   "))
        });
    let lines = guesstrix
        .collect::<Vec<String>>().join("\n");
    mat.sections[0].value = format!("            {}\n{}", counts, lines);

}
