use crate::board::{BoardState, Slot};
use crate::deck::Deck;
use crate::events::{HoverEnterEvent, HoverExitEvent};
use crate::types::CardFilter;
use crate::util::{HoverState, Hoverable};
use bevy::asset::LoadState;
use bevy::prelude::*;
use strum_macros::EnumIter;

pub const CARD_WIDTH: f32 = 352.0;
pub const CARD_HEIGHT: f32 = 512.0;
pub const CARD_SCALE: f32 = 0.2;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Deck>()
            .init_resource::<AssetsLoading>()
            .add_systems(Startup, init_load_card_assets)
            .add_systems(PostStartup, setup_cards)
            .add_systems(
                Update,
                (
                    check_assets_ready.run_if(resource_exists::<AssetsLoading>),
                    handle_hover_enter,
                    handle_hover_exit,
                ),
            );
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, EnumIter)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(PartialEq, PartialOrd, Debug, Clone, EnumIter)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Component, PartialEq, PartialOrd, Debug, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub flipped: bool,
}

impl Card {
    pub fn asset(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        let prefix = match self.suit {
            Suit::Clubs => "clubs".to_string(),
            Suit::Diamonds => "diamonds".to_string(),
            Suit::Hearts => "hearts".to_string(),
            Suit::Spades => "spades".to_string(),
        };

        let suffix = match self.rank {
            Rank::Ace => "ace".to_string(),
            Rank::Two => "02".to_string(),
            Rank::Three => "03".to_string(),
            Rank::Four => "04".to_string(),
            Rank::Five => "05".to_string(),
            Rank::Six => "06".to_string(),
            Rank::Seven => "07".to_string(),
            Rank::Eight => "08".to_string(),
            Rank::Nine => "09".to_string(),
            Rank::Ten => "10".to_string(),
            Rank::Jack => "jack".to_string(),
            Rank::Queen => "queen".to_string(),
            Rank::King => "king".to_string(),
        };

        let resource_path = format!("{}_{}.png", prefix, suffix);
        asset_server.load(resource_path)
    }

    pub fn back_asset(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        asset_server.load("back01.png")
    }
}

#[derive(Resource, Default, Debug)]
pub struct AssetsLoading(Vec<Handle<Image>>);

#[derive(Bundle)]
pub struct CardBundle {
    card: Card,
    sprite: Sprite,
    transform: Transform,
}

impl CardBundle {
    pub fn new(card: &Card, asset_server: &Res<AssetServer>, transform: Transform) -> Self {
        let image = if card.flipped {
            card.back_asset(asset_server)
        } else {
            card.asset(asset_server)
        };

        CardBundle {
            card: card.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: CARD_WIDTH * CARD_SCALE,
                    y: CARD_HEIGHT * CARD_SCALE,
                }),
                image,
                ..default()
            },
            transform,
        }
    }
}

fn init_load_card_assets(
    asset_server: Res<AssetServer>,
    deck: Res<Deck>,
    mut loading: ResMut<AssetsLoading>,
) {
    for card in deck.get_cards() {
        let handle = card.asset(&asset_server);
        loading.0.push(handle);
    }
}

fn check_assets_ready(
    mut commands: Commands,
    deck: Res<Deck>,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    let mut load_count = 0;

    for asset_handle in loading.0.iter() {
        let load_state = server.get_load_state(asset_handle.id());
        match load_state {
            Some(LoadState::Loaded) => {
                load_count += 1;
            }
            Some(LoadState::Failed(error)) => {
                warn!("Failed to load asset: {}", error.to_string());
            }
            _ => (),
        }
    }

    if load_count == deck.get_cards().len() {
        println!("Loaded all {} card assets.", load_count);
        commands.remove_resource::<AssetsLoading>();
    }
}

fn setup_cards(
    mut commands: Commands,
    mut board_state: ResMut<BoardState>,
    mut deck: ResMut<Deck>,
    server: Res<AssetServer>,
    slots: Query<(&Transform, &Slot)>,
) {
    let play_piles = &mut board_state.play_piles;
    let mut target_positions = vec![Vec3::default(); play_piles.len()];
    for (slot_transform, slot) in slots.iter() {
        if let Slot::Play(n) = slot {
            target_positions[*n as usize] = slot_transform.translation;
        }
    }
    for i in 0..play_piles.len() {
        let y_offset = (i * 40) as f32;
        for j in i..play_piles.len() {
            let Some(mut drawn_card) = deck.play() else {
                return;
            };

            if j == i {
                drawn_card.flipped = false
            }

            let transform = Transform::from_xyz(
                target_positions[j].x,
                target_positions[j].y - y_offset,
                i as f32,
            );

            commands.spawn((
                CardBundle::new(&drawn_card, &server, transform),
                Hoverable,
                HoverState::default(),
            ));

            play_piles[j].push(drawn_card);
        }
    }
}

fn handle_hover_enter(
    mut events: EventReader<HoverEnterEvent>,
    mut query: Query<&mut Transform, CardFilter>,
) {
    for HoverEnterEvent(entity) in events.read() {
        if let Ok(mut transform) = query.get_mut(*entity) {
            transform.scale = Vec3::splat(1.1);
        }
    }
}

fn handle_hover_exit(
    mut events: EventReader<HoverExitEvent>,
    mut query: Query<&mut Transform, CardFilter>,
) {
    for HoverExitEvent(entity) in events.read() {
        if let Ok(mut transform) = query.get_mut(*entity) {
            transform.scale = Vec3::splat(1.0);
        }
    }
}
