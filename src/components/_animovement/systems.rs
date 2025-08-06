use bevy::prelude::*;

use super::{animation::Animation, movement::Movement};

pub fn update_animation_from_movement(
    mut player_query: Query<(&Movement, &mut Animation)>,
) {
    for (movement, mut animation) in &mut player_query {
        animation.update_from_point(movement.intent);
    }
}
