pub mod cursor;
pub mod debug;
pub mod dragging;
pub mod flipping;
pub mod hovering;
pub mod moveto;

use crate::utils::cursor::{Cursor, update_cursor, update_cursor_icon};
use crate::utils::debug::{DebugMode, toggle_debug_mode};
use crate::utils::dragging::{drag_system, start_drag_system, stop_drag_system};
use crate::utils::flipping::handle_flip;
use crate::utils::hovering::{
    hover_card_system, hover_deck_system, hover_play_slot_system, reset_hover_flags,
};
use crate::utils::moveto::handle_move_to;
use bevy::prelude::*;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .init_resource::<DebugMode>()
            .add_systems(PreUpdate, update_cursor)
            .add_systems(
                Update,
                (
                    update_cursor_icon,
                    hover_card_system,
                    hover_play_slot_system,
                    hover_deck_system,
                    reset_hover_flags,
                    handle_flip,
                    handle_move_to,
                    start_drag_system,
                    drag_system,
                    stop_drag_system,
                    toggle_debug_mode,
                ),
            );
    }
}

pub fn in_region(pos: Vec2, region_pos: Vec2, region_size: Vec2) -> bool {
    let half_size = region_size / 2.0;
    let min = region_pos - half_size;
    let max = region_pos + half_size;
    pos.x >= min.x && pos.x <= max.x && pos.y >= min.y && pos.y <= max.y
}
