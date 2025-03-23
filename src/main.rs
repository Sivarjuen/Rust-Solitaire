mod board;
mod card;
mod config;
mod deck;

use board::BoardPlugin;
use card::CardPlugin;

use bevy::prelude::*;
use config::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Solitaire by SIV".to_string(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((BoardPlugin, CardPlugin))
        .insert_resource(ClearColor(BG_COLOUR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
