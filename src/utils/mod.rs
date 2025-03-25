pub mod dragging;
pub mod flipping;
pub mod hovering;
pub mod moveto;

use crate::utils::dragging::{drag_system, start_drag_system, stop_drag_system};
use crate::utils::flipping::handle_flip;
use crate::utils::hovering::{hover_card_system, reset_hover_flags};
use crate::utils::moveto::handle_move_to;
use bevy::prelude::*;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                hover_card_system,
                reset_hover_flags,
                handle_flip,
                handle_move_to,
                start_drag_system,
                drag_system,
                stop_drag_system,
            ),
        );
    }
}
