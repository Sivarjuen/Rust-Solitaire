use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Gizmos, KeyCode, Res, ResMut, Resource};

#[derive(Resource, Default)]
pub struct DebugMode {
    pub enabled: bool,
}

pub fn toggle_debug_mode(mut debug_mode: ResMut<DebugMode>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Tab) {
        debug_mode.enabled = !debug_mode.enabled;
    }
}

pub fn draw_debug_box(position: Vec2, size: Vec2, color: Color, gizmos: &mut Gizmos) {
    let half = size / 2.0;

    let top_left = position + Vec2::new(-half.x, half.y);
    let top_right = position + Vec2::new(half.x, half.y);
    let bottom_right = position + Vec2::new(half.x, -half.y);
    let bottom_left = position + Vec2::new(-half.x, -half.y);

    gizmos.line_2d(top_left, top_right, color);
    gizmos.line_2d(top_right, bottom_right, color);
    gizmos.line_2d(bottom_right, bottom_left, color);
    gizmos.line_2d(bottom_left, top_left, color);
}
