use crate::board::{Col, DeckPosition, Home, Slot};
use crate::events::{HoverEnterEvent, HoverExitEvent};
use crate::types::{CardFilter, CardHoverItem, CardSimpleHoverItem, HoverItem};
use crate::utils::cursor::Cursor;
use crate::utils::debug::draw_debug_box;
use crate::utils::{DebugMode, in_region};
use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component)]
pub struct Hoverable;

#[derive(Component, Default)]
pub struct HoverState {
    pub hovering: bool,
}

pub fn hover_card_system(
    cursor: Res<Cursor>,
    mut card_q: Query<CardHoverItem, CardFilter>,
    mut hover_enter_writer: EventWriter<HoverEnterEvent>,
    mut hover_exit_writer: EventWriter<HoverExitEvent>,
) {
    if let Some(cursor_world) = cursor.position {
        let mut candidates = vec![];

        for (entity, transform, sprite, _, _) in card_q.iter_mut() {
            let position = transform.translation.truncate();
            let size = sprite.custom_size.unwrap_or(Vec2::ONE);
            let is_hovering = in_region(cursor_world, position, size);

            if is_hovering {
                candidates.push((transform.translation.z, entity));
            }
        }

        candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        if let Some((_, top_entity)) = candidates.first() {
            for (entity, _, _, mut hover_state, _) in card_q.iter_mut() {
                if entity == *top_entity {
                    if !hover_state.hovering {
                        hover_state.hovering = true;
                        hover_enter_writer.send(HoverEnterEvent(entity));
                    }
                } else if hover_state.hovering {
                    hover_state.hovering = false;
                    hover_exit_writer.send(HoverExitEvent(entity));
                }
            }
        } else {
            for (entity, _, _, mut hover_state, _) in card_q.iter_mut() {
                if hover_state.hovering {
                    hover_state.hovering = false;
                    hover_exit_writer.send(HoverExitEvent(entity));
                }
            }
        }
    }
}

// Handle hover exit if cursor leaves window
pub fn reset_hover_flags(
    cursor: Res<Cursor>,
    mut query: Query<CardSimpleHoverItem, CardFilter>,
    mut hover_exit_writer: EventWriter<HoverExitEvent>,
) {
    if cursor.position.is_none() {
        for (entity, mut hover_state) in query.iter_mut() {
            if hover_state.hovering {
                hover_state.hovering = false;
                hover_exit_writer.send(HoverExitEvent(entity));
            }
        }
    }
}

pub fn hover_deck_system(
    cursor: Res<Cursor>,
    mut deck_q: Query<HoverItem, (With<DeckPosition>, With<Hoverable>, With<Slot>)>,
) {
    let Some(cursor_world) = cursor.position else {
        return;
    };

    for (_, transform, sprite, mut hover_state) in deck_q.iter_mut() {
        let position = transform.translation.truncate();
        let size = sprite.custom_size.unwrap_or(Vec2::ONE);
        let is_hovering = in_region(cursor_world, position, size);

        if is_hovering && !hover_state.hovering {
            hover_state.hovering = true;
        } else if !is_hovering && hover_state.hovering {
            hover_state.hovering = false;
        }
    }
}

pub fn hover_play_slot_system(
    cursor: Res<Cursor>,
    mut slot_q: Query<
        (HoverItem, Option<&Col>, Option<&Home>),
        (With<Hoverable>, With<Slot>, Or<(With<Col>, With<Home>)>),
    >,
    mut hover_enter_writer: EventWriter<HoverEnterEvent>,
    mut hover_exit_writer: EventWriter<HoverExitEvent>,
    mut gizmos: Gizmos,
    debug_mode: ResMut<DebugMode>,
) {
    let Some(cursor_world) = cursor.position else {
        return;
    };

    for ((entity, transform, sprite, mut hover_state), col, home) in slot_q.iter_mut() {
        let mut position = transform.translation.truncate();
        let mut size = sprite.custom_size.unwrap_or(Vec2::ONE);
        if col.is_some() {
            position.y -= size.y * 2f32;
            size.x *= 1.3;
            size.y *= 5f32;
        }
        if home.is_some() {
            size *= 1.3;
        }

        if debug_mode.enabled {
            draw_debug_box(position, size, Color::srgb(1f32, 0f32, 0f32), &mut gizmos);
        }
        let is_hovering = in_region(cursor_world, position, size);

        if is_hovering && !hover_state.hovering {
            hover_state.hovering = true;
            hover_enter_writer.send(HoverEnterEvent(entity));
        } else if !is_hovering && hover_state.hovering {
            hover_state.hovering = false;
            hover_exit_writer.send(HoverExitEvent(entity));
        }
    }
}
