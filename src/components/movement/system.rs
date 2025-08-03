#![allow(dead_code)]

use bevy::prelude::*;

use super::controller::MovementController;

pub fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut Transform)>,
) {
    for (controller, mut transform) in &mut movement_query {
        let velocity = controller.max_speed * controller.intent;

        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}
