use crate::board::{BoardState, Col, DeckPosition, DrawPosition, Slot};
use crate::deck::Deck;
use crate::events::{HoverEnterEvent, HoverExitEvent};
use crate::types::{
    CardFilter, CardHoverItem, DeckCardFilter, DeckSlotFilter, DrawCardFilter, DrawSlotFilter,
};
use crate::utils::cursor::Cursor;
use crate::utils::dragging::Draggable;
use crate::utils::flipping::Flipping;
use crate::utils::hovering::{HoverState, Hoverable};
use crate::utils::moveto::MoveTo;
use bevy::asset::LoadState;
use bevy::prelude::*;
use std::f32::consts::PI;
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
            .add_systems(PostStartup, (setup_cards, setup_deck_cards).chain())
            .add_systems(
                Update,
                (
                    check_assets_ready.run_if(resource_exists::<AssetsLoading>),
                    handle_hover_enter,
                    handle_hover_exit,
                    handle_deck_click,
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
    slots: Query<(&Transform, &Col), With<Slot>>,
) {
    let play_piles = &mut board_state.play_piles;
    let mut target_positions = vec![Vec3::default(); play_piles.len()];

    for (slot_transform, slot) in slots.iter() {
        let index = slot.0 as usize;
        target_positions[index] = slot_transform.translation;
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
                Col(j as u32),
                Draggable,
            ));

            play_piles[j].push(drawn_card);
        }
    }
}

fn setup_deck_cards(
    mut commands: Commands,
    deck: Res<Deck>,
    server: Res<AssetServer>,
    slots: Query<&Transform, DeckSlotFilter>,
) {
    let deck_position = slots.single().translation;

    let cards = deck.get_cards();
    for (i, card) in cards.iter().enumerate() {
        let transform = Transform::from_xyz(deck_position.x, deck_position.y, i as f32);

        commands.spawn((
            CardBundle::new(card, &server, transform),
            Hoverable,
            HoverState::default(),
            DeckPosition,
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_deck_click(
    mut commands: Commands,
    mut deck: ResMut<Deck>,
    deck_slot: Query<(&Transform, &Sprite), DeckSlotFilter>,
    draw_slot: Query<&Transform, DrawSlotFilter>,
    input: Res<ButtonInput<MouseButton>>,
    mut deck_card_q: Query<CardHoverItem, DeckCardFilter>,
    mut draw_card_q: Query<CardHoverItem, DrawCardFilter>,
    mut hover_exit_writer: EventWriter<HoverExitEvent>,
    server: Res<AssetServer>,
    cursor: Res<Cursor>,
) {
    if input.just_pressed(MouseButton::Left) {
        let reset_deck = deck.is_empty() && !deck.get_drawn_cards().is_empty();
        if reset_deck {
            let (deck_transform, deck_sprite) = deck_slot.single();
            let deck_position = deck_transform.translation;
            if let Some(cursor_position) = cursor.position {
                if let Some(sprite_size) = deck_sprite.custom_size {
                    let half_width = sprite_size.x / 2.0;
                    let half_height = sprite_size.y / 2.0;

                    if !(cursor_position.x > deck_position.x - half_width
                        && cursor_position.x < deck_position.x + half_width
                        && cursor_position.y > deck_position.y - half_height
                        && cursor_position.y < deck_position.y + half_height)
                    {
                        return;
                    }
                }
            }
            for (entity, mut transform, mut sprite, _, mut card) in draw_card_q.iter_mut() {
                let current_z = transform.translation.z;
                transform.translation = Vec3::new(
                    deck_position.x,
                    deck_position.y,
                    deck.get_drawn_cards().len() as f32 - current_z,
                );

                commands
                    .entity(entity)
                    .remove::<DrawPosition>()
                    .remove::<MoveTo>()
                    .remove::<Draggable>()
                    .insert(DeckPosition);
                card.flipped = true;
                sprite.image = card.back_asset(&server);
            }
            deck.reset();
        } else {
            for (entity, mut transform, _, mut hover_state, card) in deck_card_q.iter_mut() {
                if hover_state.hovering {
                    let Some(top_card) = deck.draw() else {
                        continue;
                    };
                    if top_card != *card {
                        continue;
                    }

                    hover_state.hovering = false;
                    hover_exit_writer.send(HoverExitEvent(entity));

                    let draw_position = draw_slot.single().translation;
                    transform.translation.z = 100.0;
                    let target_position = Vec3::new(
                        draw_position.x,
                        draw_position.y,
                        deck.get_drawn_cards().len() as f32,
                    );

                    commands
                        .entity(entity)
                        .remove::<DeckPosition>()
                        .insert(DrawPosition)
                        .insert(MoveTo {
                            target: target_position,
                            speed: 400.0,
                        })
                        .insert(Draggable)
                        .insert(Flipping {
                            speed: PI * 3.0,
                            flipped: false,
                            progress: 0.0,
                        });
                    break;
                }
            }
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
