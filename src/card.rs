use bevy::asset::LoadState;
use bevy::prelude::*;
use strum_macros::EnumIter;

use crate::deck::Deck;

pub const CARD_WIDTH: f32 = 352.0;
pub const CARD_HEIGHT: f32 = 512.0;
pub const CARD_SCALE: f32 = 0.2;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Deck>()
            .init_resource::<AssetsLoading>()
            .add_systems(Startup, init_load_card_assets)
            .add_systems(
                Update,
                (
                    check_assets_ready.run_if(resource_exists::<AssetsLoading>),
                    change_sprite_image,
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
    pub fn new(card: Card, asset_server: &Res<AssetServer>, transform: Transform) -> Self {
        CardBundle {
            card: card.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: CARD_WIDTH * CARD_SCALE,
                    y: CARD_HEIGHT * CARD_SCALE,
                }),
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
                flipped: false,
            },
            &server,
            Transform::default(),
        ));
        commands.spawn(CardBundle::new(
            Card {
                rank: Rank::QUEEN,
                suit: Suit::Spades,
                flipped: false,
            },
            &server,
            Transform::from_xyz(100.0, 0.0, 0.0),
        ));
    }
}

// FIXME: this is temporary
fn change_sprite_image(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Sprite, &mut Card)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (mut sprite, mut card) in query.iter_mut() {
            let mut new_texture = card.asset(&asset_server);
            if card.flipped {
                new_texture = card.back_asset(&asset_server);
            }
            card.flipped = !card.flipped;
            sprite.image = new_texture.clone();
        }

        println!("Sprite image changed!");
    }
}
