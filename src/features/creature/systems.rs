#![allow(dead_code)]

use bevy::{input::gamepad::GamepadEvent, prelude::*};

use super::markers::Creature;
use crate::features::creature::bundles::npc_bundle;
use crate::shared::data_structures::{ChunkToGrid, PrimitiveRect};
use crate::shared::z_levels::ZLevel;
use crate::{
    components::{animation::Animation, movement::Movement},
    features::creature::bundles::player_bundle,
};

pub fn record_creature_wasd_input(
    input: Res<ButtonInput<KeyCode>>,
    mut creature_query: Query<(&mut Movement, &Creature)>,
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
    for (mut movement, creature) in &mut creature_query {
        if creature.is_controlled {
            movement.intent = intent;
        }
    }
}

pub fn tick_creature_animation_timer(
    time: Res<Time>,
    mut query: Query<&mut Animation, With<Creature>>,
) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

pub fn update_creature_sprite_animation(
    mut query: Query<(&Animation, &mut Sprite), With<Creature>>,
) {
    for (animation, mut sprite) in &mut query {
        // Is switched to next frame. Syncing with atlas frame
        if animation.changed() {
            let atlas = sprite.texture_atlas.as_mut().unwrap();

            atlas.index = usize::from(animation.frame());
        }
    }
}

pub fn update_creature_animation_state(
    mut player_query: Query<(&Movement, &mut Sprite, &mut Animation), With<Creature>>,
) {
    for (movement, mut _sprite, mut animation) in &mut player_query {
        animation.update_from_point(movement.intent);
    }
}
