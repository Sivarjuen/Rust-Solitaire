use bevy::math::Vec3;
use bevy::prelude::*;

#[derive(Component)]
pub struct MoveTo {
    pub target: Vec3,
    pub speed: f32,
}

pub fn handle_move_to(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &MoveTo)>,
) {
    for (entity, mut transform, move_to) in query.iter_mut() {
        let direction = move_to.target - transform.translation;
        let distance = direction.length();

        if distance < 10.0 {
            transform.translation = move_to.target;
            commands.entity(entity).remove::<MoveTo>();
        } else {
            let movement = direction.normalize() * move_to.speed * time.delta_secs();
            transform.translation += movement;
        }
    }
}
