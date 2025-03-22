use bevy::asset::LoadState;
use bevy::prelude::*;
use strum_macros::EnumIter;

use crate::deck::Deck;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Deck>()
            .init_resource::<AssetsLoading>()
            .add_systems(Startup, init_load_card_assets)
            .add_systems(
                Update,
                check_assets_ready.run_if(resource_exists::<AssetsLoading>),
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
    ACE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}

#[derive(Component, PartialEq, PartialOrd, Debug, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
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
            Rank::ACE => "ace".to_string(),
            Rank::TWO => "02".to_string(),
            Rank::THREE => "03".to_string(),
            Rank::FOUR => "04".to_string(),
            Rank::FIVE => "05".to_string(),
            Rank::SIX => "06".to_string(),
            Rank::SEVEN => "07".to_string(),
            Rank::EIGHT => "08".to_string(),
            Rank::NINE => "09".to_string(),
            Rank::TEN => "10".to_string(),
            Rank::JACK => "jack".to_string(),
            Rank::QUEEN => "queen".to_string(),
            Rank::KING => "king".to_string(),
        };

        let resource_path = format!("{}_{}.png", prefix, suffix);
        asset_server.load(resource_path)
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
    pub fn new(card: Card, asset_server: &Res<AssetServer>, transform: Transform) -> Self {
        CardBundle {
            card: card.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 200.0, y: 300.0 }),
                image: card.asset(asset_server),
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

        commands.spawn(CardBundle::new(
            Card {
                rank: Rank::KING,
                suit: Suit::Diamonds,
            },
            &server,
            Transform::default(),
        ));
        commands.spawn(CardBundle::new(
            Card {
                rank: Rank::QUEEN,
                suit: Suit::Spades,
            },
            &server,
            Transform::from_xyz(100.0, 0.0, 0.0),
        ));
    }
}
