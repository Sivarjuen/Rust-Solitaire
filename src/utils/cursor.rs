use bevy::prelude::*;

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
