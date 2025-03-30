use bevy::prelude::*;

use crate::card::{CARD_HEIGHT, CARD_SCALE, CARD_WIDTH, Card};
use crate::utils::hovering::{HoverState, Hoverable};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardState>()
            .add_systems(Startup, (setup_background, setup_slots));
    }
}

#[derive(Resource, PartialEq, Debug)]
pub struct BoardState {
    pub home_piles: Vec<Vec<Card>>,
    pub play_piles: Vec<Vec<Card>>,
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            home_piles: (0..4).map(|_| Vec::new()).collect(),
            play_piles: (0..7).map(|_| Vec::new()).collect(),
        }
    }
}

#[derive(Component, PartialEq)]
pub struct DeckPosition;

#[derive(Component, PartialEq)]
pub struct DrawPosition;

#[derive(Component, PartialEq)]
pub struct Home(pub u32);

#[derive(Component, PartialEq)]
pub struct Col(pub u32);

#[derive(Component, PartialEq)]
pub struct Slot;

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("bg.png");
    commands.spawn((
        Sprite {
            custom_size: None,
            image: texture_handle,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -2.0),
        GlobalTransform::default(),
    ));
}

fn setup_slots(mut commands: Commands, windows: Query<&Window>, asset_server: Res<AssetServer>) {
    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();

    let slot_count = 7;
    let slots_width = window_width * 0.8;
    let spacing = slots_width / (slot_count + 1) as f32;
    let top_row_y = (window_height / 2.0) - (CARD_HEIGHT * CARD_SCALE);
    let bottom_row_y = top_row_y - (CARD_HEIGHT * CARD_SCALE * 1.5);

    let texture_handle = asset_server.load("slot.png");

    for i in 0..slot_count {
        let x = -slots_width / 2.0 + spacing * (i + 1) as f32;

        commands.spawn((
            Sprite {
                custom_size: Some(Vec2 {
                    x: CARD_WIDTH * CARD_SCALE,
                    y: CARD_HEIGHT * CARD_SCALE,
                }),
                image: texture_handle.clone(),
                ..default()
            },
            Transform::from_xyz(x, bottom_row_y, -1.0),
            Slot,
            Col(i),
            Hoverable,
            HoverState::default(),
            GlobalTransform::default(),
        ));

        match i {
            0 => {
                commands.spawn((
                    Sprite {
                        custom_size: Some(Vec2 {
                            x: CARD_WIDTH * CARD_SCALE,
                            y: CARD_HEIGHT * CARD_SCALE,
                        }),
                        image: texture_handle.clone(),
                        ..default()
                    },
                    Transform::from_xyz(x, top_row_y, -1.0),
                    Slot,
                    DeckPosition,
                    Hoverable,
                    HoverState::default(),
                    GlobalTransform::default(),
                ));
            }
            1 => {
                commands.spawn((
                    Transform::from_xyz(x, top_row_y, -1.0),
                    Slot,
                    DrawPosition,
                    GlobalTransform::default(),
                ));
            }
            3..=6 => {
                commands.spawn((
                    Sprite {
                        custom_size: Some(Vec2 {
                            x: CARD_WIDTH * CARD_SCALE,
                            y: CARD_HEIGHT * CARD_SCALE,
                        }),
                        image: texture_handle.clone(),
                        ..default()
                    },
                    Transform::from_xyz(x, top_row_y, -1.0),
                    Slot,
                    Home(i),
                    Hoverable,
                    HoverState::default(),
                    GlobalTransform::default(),
                ));
            }
            _ => (),
        }
    }
}
