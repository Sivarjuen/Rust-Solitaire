use crate::card::Card;
use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(Component)]
pub struct Flipping {
    pub speed: f32,
    pub flipped: bool,
    pub progress: f32,
}

pub fn handle_flip(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Flipping,
        &mut Sprite,
        &mut Card,
    )>,
    server: Res<AssetServer>,
) {
    for (entity, mut transform, mut flipping, mut sprite, mut card) in query.iter_mut() {
        let delta_rotation = flipping.speed * time.delta_secs();
        flipping.progress += delta_rotation;
        transform.rotation = Quat::from_rotation_y(flipping.progress);

        if !flipping.flipped && flipping.progress >= FRAC_PI_2 {
            sprite.image = card.asset(&server);
            card.flipped = false;
            flipping.flipped = true;

            transform.scale.x *= -1.0;
        }

        if flipping.progress >= PI {
            transform.rotation = Quat::IDENTITY;
            transform.scale.x = transform.scale.x.abs();
            commands.entity(entity).remove::<Flipping>();
        }
    }
}
