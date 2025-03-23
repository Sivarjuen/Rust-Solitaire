use bevy::prelude::*;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HoverEnterEvent>()
            .add_event::<HoverExitEvent>();
    }
}

#[derive(Event)]
pub struct HoverEnterEvent(pub Entity);
#[derive(Event)]
pub struct HoverExitEvent(pub Entity);
