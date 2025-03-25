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
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    mut q_cards: Query<(Entity, &GlobalTransform, &mut Transform, &HoverState), With<Draggable>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(world_position) = camera
            .viewport_to_world(camera_transform, cursor_position)
            .map(|ray| ray.origin.truncate())
        {
            for (entity, g_transform, mut transform, hover_state) in q_cards.iter_mut() {
                if hover_state.hovering {
                    let card_pos = g_transform.translation().truncate();
                    let offset = card_pos - world_position;

                    commands.entity(entity).insert(Dragging {
                        original_position: g_transform.translation(),
                        offset,
                    });

                    transform.translation.z += 100.0;

                    break;
                }
            }
        }
    }
}

pub fn drag_system(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(&mut Transform, &Dragging)>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(world_position) = camera
            .viewport_to_world(camera_transform, cursor_position)
            .map(|ray| ray.origin.truncate())
        {
            for (mut transform, dragging) in query.iter_mut() {
                let new_position = world_position + dragging.offset;
                transform.translation.x = new_position.x;
                transform.translation.y = new_position.y;
            }
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
