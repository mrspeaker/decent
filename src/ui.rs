use bevy::prelude::*;

#[derive(Component)]
pub struct TitleText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_systems(Startup, setup);
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
                    "...Decent",
                    TextStyle {
                        font_size: 20.,
                        ..default()
                    },
                ).with_style(Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(50.0),
                    right: Val::Percent(50.0),
                    ..default()
                }),
                TitleText));

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
