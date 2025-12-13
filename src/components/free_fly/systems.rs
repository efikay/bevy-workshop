use bevy::prelude::*;

use super::*;

pub fn record_free_fly_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_transform: Single<&mut Transform, With<component::FreeFly>>,
    time: Res<Time>,
) {
    let mut intent = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        intent.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        intent.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        intent.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        intent.x += 1.0;
    }
}
