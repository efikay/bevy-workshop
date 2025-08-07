#![allow(dead_code)]

use bevy::prelude::*;

use crate::shared::common_markers::WASD;

use super::component::Movement;

pub fn apply_movement(time: Res<Time>, mut movement_query: Query<(&Movement, &mut Transform)>) {
    for (movement, mut transform) in &mut movement_query {
        let velocity = movement.max_speed * movement.intent;

        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}

pub fn record_wasd_input(
    input: Res<ButtonInput<KeyCode>>,
    mut creature_query: Query<&mut Movement, With<WASD>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
    // This should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut movement in &mut creature_query {
        movement.intent = intent;
    }
}

pub fn record_gamepad_movement_input(
    gamepads: Query<&Gamepad>,
    mut creature: Query<&mut Movement, With<WASD>>,
) {
    const MIN_AXIS_SENSITIVITY: f32 = 0.2;

    for gamepad in &gamepads {
        // Collect directional input.
        let mut intent = Vec2::ZERO;
        if gamepad.pressed(GamepadButton::DPadUp) {
            intent.y += 1.0;
        }
        if gamepad.pressed(GamepadButton::DPadDown) {
            intent.y -= 1.0;
        }
        if gamepad.pressed(GamepadButton::DPadLeft) {
            intent.x -= 1.0;
        }
        if gamepad.pressed(GamepadButton::DPadRight) {
            intent.x += 1.0;
        }

        if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX) {
            if left_stick_x.abs() > MIN_AXIS_SENSITIVITY {
                intent.x += left_stick_x;
            }
        }
        if let Some(left_stick_y) = gamepad.get(GamepadAxis::LeftStickY) {
            if left_stick_y.abs() > MIN_AXIS_SENSITIVITY {
                intent.y += left_stick_y;
            }
        }

        for mut movement in &mut creature {
            movement.intent = intent.clamp(Vec2::NEG_ONE, Vec2::ONE);
        }
    }
}
