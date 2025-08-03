#![allow(dead_code)]

use bevy::{input::gamepad::GamepadEvent, prelude::*};

use super::marker::Creature;
use crate::components::chess_floor::make_sprite_bundles;
use crate::shared::z_levels::ZLevel;
use crate::{
    components::{animation::Animation, movement::Movement},
    features::creature::bundles::player_bundle,
};

pub fn debug_spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(player_bundle(
        asset_server.clone(),
        &mut texture_atlas_layouts,
    ));
}

pub fn debug_camera_seek_player(
    mut camera_transform: Single<&mut Transform, (With<Camera2d>, Without<Creature>)>,
    player_transform: Single<&Transform, (With<Creature>, Without<Camera2d>)>,
) {
    camera_transform.translation = player_transform.translation.clone();
}

pub fn record_creature_wasd_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut Movement, With<Creature>>,
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
    for mut controller in &mut controller_query {
        controller.intent = intent;
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
    for (controller, mut _sprite, mut animation) in &mut player_query {
        // TODO make configurable flips
        // let dx = controller.intent.x;
        // if dx != 0.0 {
        // sprite.flip_x = dx < 0.0;
        // }

        animation.update_from_point(controller.intent);
    }
}
