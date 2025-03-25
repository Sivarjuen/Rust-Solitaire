use crate::board::{Col, DeckPosition, DrawPosition, Home, Slot};
use crate::card::Card;
use crate::utils::hovering::{HoverState, Hoverable};
use bevy::prelude::*;

pub type CardFilter = (With<Hoverable>, With<Card>, Without<Slot>);
pub type DeckCardFilter = (
    With<Hoverable>,
    With<Card>,
    With<DeckPosition>,
    Without<Slot>,
    Without<DrawPosition>,
);
pub type DrawCardFilter = (
    With<Hoverable>,
    With<Card>,
    With<DrawPosition>,
    Without<Slot>,
    Without<DeckPosition>,
);

pub type DrawSlotFilter = (With<Slot>, With<DrawPosition>);
pub type DeckSlotFilter = (With<Slot>, With<DeckPosition>);
pub type _ColSlotFilter = (With<Slot>, With<Col>);
pub type _HomeSlotFilter = (With<Slot>, With<Home>);

pub type CardSimpleHoverItem<'w> = (Entity, &'w mut HoverState);

pub type CardHoverItem<'w> = (
    Entity,
    &'w mut Transform,
    &'w mut Sprite,
    &'w mut HoverState,
    &'w mut Card,
);
