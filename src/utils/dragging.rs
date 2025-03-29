use crate::board::BoardState;
use crate::card::Card;
use crate::utils::cursor::Cursor;
use crate::utils::hovering::HoverState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Draggable;

#[derive(Component)]
pub struct Dragging {
    original_position: Vec3,
    offset: Vec2,
}

pub fn start_drag_system(
    buttons: Res<ButtonInput<MouseButton>>,
    board_state: Res<BoardState>,
    cursor: Res<Cursor>,
    mut commands: Commands,
    mut q_cards: Query<
        (Entity, &GlobalTransform, &mut Transform, &HoverState, &Card),
        With<Draggable>,
    >,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(world_position) = cursor.position {
        let mut target_cards = vec![];
        let pile_groups = [&board_state.play_piles, &board_state.home_piles];

        'outer: for (_, _, _, hover_state, card) in q_cards.iter() {
            if hover_state.hovering {
                for piles in pile_groups {
                    for pile in piles {
                        let mut add_card = false;
                        for pile_card in pile {
                            if add_card {
                                target_cards.push(pile_card);
                            } else if card == pile_card {
                                target_cards.push(pile_card);
                                add_card = true
                            }
                        }
                        if add_card {
                            break 'outer;
                        }
                    }
                }
            }
        }

        for (entity, g_transform, mut transform, hover_state, card) in q_cards.iter_mut() {
            if target_cards.contains(&card) || (target_cards.is_empty() && hover_state.hovering) {
                let card_pos = g_transform.translation().truncate();
                let offset = card_pos - world_position;

                commands.entity(entity).insert(Dragging {
                    original_position: g_transform.translation(),
                    offset,
                });

                transform.translation.z += 100.0;
            }
        }
    }
}

pub fn drag_system(
    buttons: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    mut query: Query<(&mut Transform, &Dragging)>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    if let Some(world_position) = cursor.position {
        for (mut transform, dragging) in query.iter_mut() {
            let new_position = world_position + dragging.offset;
            transform.translation.x = new_position.x;
            transform.translation.y = new_position.y;
        }
    }
}

pub fn stop_drag_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Dragging)>,
) {
    if buttons.just_released(MouseButton::Left) {
        for (entity, mut transform, dragging) in query.iter_mut() {
            transform.translation = dragging.original_position;
            commands.entity(entity).remove::<Dragging>();
        }
    }
}
