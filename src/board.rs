use bevy::prelude::*;

use crate::card::{CARD_HEIGHT, CARD_SCALE, CARD_WIDTH, Card};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardState>()
            .add_systems(Startup, (setup_background, setup_slots));
    }
}

#[derive(Resource, PartialEq, Debug)]
pub struct BoardState {
    home_piles: Vec<Vec<Card>>,
    play_piles: Vec<Vec<Card>>,
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            home_piles: (0..4).map(|_| Vec::new()).collect(),
            play_piles: (0..7).map(|_| Vec::new()).collect(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SlotId {
    Deck,
    Draw,
    Home(u32),
    Play(u32),
}

#[derive(Component)]
pub struct Slot(pub SlotId);

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
            Slot(SlotId::Play(i)),
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
                    Slot(SlotId::Deck),
                    GlobalTransform::default(),
                ));
            }
            1 => {
                commands.spawn((
                    Transform::from_xyz(x, top_row_y, -1.0),
                    Slot(SlotId::Draw),
                    GlobalTransform::default(),
                ));
            }
            3 | 4 | 5 | 6 => {
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
                    Slot(SlotId::Home(i)),
                    GlobalTransform::default(),
                ));
            }
            _ => (),
        }
    }
}
