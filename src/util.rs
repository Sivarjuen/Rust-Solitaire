use bevy::prelude::*;
use crate::card::Card;
use crate::events::{HoverEnterEvent, HoverExitEvent};

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (hover_card_system, reset_hover_flags));
    }
}

#[derive(Component)]
pub struct Hoverable;

#[derive(Component, Default)]
pub struct HoverState {
    hovering: bool,
}

fn hover_card_system(
    window: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut card_q: Query<(Entity, &GlobalTransform, &Sprite, &mut HoverState), (With<Hoverable>, With<Card>)>,
    mut hover_enter_writer: EventWriter<HoverEnterEvent>,
    mut hover_exit_writer: EventWriter<HoverExitEvent>,
) {
    let window = window.single();

    if let Some(cursor_position) = window.cursor_position() {
        let (camera, camera_transform) = camera_q.single();

        if let Ok(world_position) = camera.viewport_to_world(camera_transform, cursor_position) {
            let cursor_world = world_position.origin.truncate();

            let mut candidates = vec![];

            for (entity, transform, sprite, _) in card_q.iter_mut() {
                let position = transform.translation().truncate();
                let size = sprite.custom_size.unwrap_or(Vec2::ONE);
                let half_size = size / 2.0;
                let min = position - half_size;
                let max = position + half_size;

                let is_hovering = cursor_world.x >= min.x
                    && cursor_world.x <= max.x
                    && cursor_world.y >= min.y
                    && cursor_world.y <= max.y;

                if is_hovering {
                    candidates.push((transform.translation().z, entity));
                }
            }

            candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

            if let Some((_, top_entity)) = candidates.first() {
                for (entity, _, _, mut hover_state) in card_q.iter_mut() {
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
                for (entity, _, _, mut hover_state) in card_q.iter_mut() {
                    if hover_state.hovering {
                        hover_state.hovering = false;
                        hover_exit_writer.send(HoverExitEvent(entity));
                    }
                }
            }
        }
    }
}

// Handle hover exit if cursor leaves window
fn reset_hover_flags(
    mut query: Query<(Entity, &mut HoverState), (With<Hoverable>, With<Card>)>,
    windows: Query<&Window>,
    mut hover_exit_writer: EventWriter<HoverExitEvent>,
) {
    let window = windows.single();
    if window.cursor_position().is_none() {
        for (entity, mut hover_state) in query.iter_mut() {
            if hover_state.hovering {
                hover_state.hovering = false;
                hover_exit_writer.send(HoverExitEvent(entity));
            }
        }
    }
}