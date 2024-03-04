mod camera;
mod player;
mod physics;
mod laxer;
mod particles;
mod target;
mod game;
mod despawn;
mod ui;
mod state;
mod scene;

use camera::CameraPlugin;
use scene::ScenePlugin;
use player::PlayerPlugin;
use physics::PhysicsPlugin;
use particles::ParticlePlugin;
use laxer::LaxerPlugin;
use target::TargetPlugin;
use game::GamePlugin;
use despawn::DespawnPlugin;
use ui::UIPlugin;
use state::GameState;

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins((
            DefaultPlugins,
            DespawnPlugin,
            CameraPlugin,
            ScenePlugin,
            GamePlugin,
            PlayerPlugin,
            LaxerPlugin,
            TargetPlugin,
            PhysicsPlugin,
            ParticlePlugin,
            UIPlugin,
        ))
        .add_systems(Startup, cursor_grab)
        .add_systems(Update, (cursor_ungrab, draw_gizmos))
        .add_systems(OnEnter(GameState::Menu), testa)
        .run();
}

fn testa() {
    info!("In menooo");
}

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

fn cursor_ungrab(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        let mut primary_window = q_windows.single_mut();
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    }
}

fn draw_gizmos(
    mut gizmos: Gizmos,
) {
    gizmos.arrow(Vec3::ZERO, Vec3::Y * 1.0, Color::BLUE);
    gizmos.arrow(Vec3::ZERO, Vec3::X * 1.0, Color::RED);
    gizmos.arrow(Vec3::ZERO, Vec3::Z * 1.0, Color::GREEN);
    gizmos.cuboid(
        Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(1.0)),
        Color::BLACK,
    );
}
