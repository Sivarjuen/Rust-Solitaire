use crate::board::DeckPosition;
use crate::card::Card;
use crate::utils::hovering::HoverState;
use bevy::prelude::*;
use bevy::winit::WinitWindows;
use winit::window::{Cursor as WinitCursor, CursorIcon};

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct Cursor {
    pub position: Option<Vec2>,
}

pub fn update_cursor(
    mut cursor_world: ResMut<Cursor>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = windows.single();

    let mut found_cursor_position = false;
    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(world_position) = camera.viewport_to_world(camera_transform, cursor_position) {
            cursor_world.position = Some(world_position.origin.truncate());
            found_cursor_position = true;
        }
    }

    if !found_cursor_position {
        cursor_world.position = None;
    }
}

pub fn update_cursor_icon(
    winit_windows: NonSend<WinitWindows>,
    windows: Query<Entity, With<Window>>,
    query: Query<&HoverState, Or<(With<Card>, With<DeckPosition>)>>,
) {
    let window_entity = windows.single();
    let Some(winit_window) = winit_windows.get_window(window_entity) else {
        return;
    };

    let hovered = query.iter().any(|h| h.hovering);

    let icon = if hovered {
        CursorIcon::Pointer
    } else {
        CursorIcon::Default
    };

    winit_window.set_cursor(WinitCursor::Icon(icon));
}
