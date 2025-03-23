use crate::card::Card;
use crate::util::{HoverState, Hoverable};
use bevy::prelude::*;

pub type CardFilter = (With<Hoverable>, With<Card>);

pub type CardSimpleHoverItem<'w> = (Entity, &'w mut HoverState);

pub type CardHoverItem<'w> = (Entity, &'w GlobalTransform, &'w Sprite, &'w mut HoverState);
