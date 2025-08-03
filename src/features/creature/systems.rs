#![allow(dead_code)]

use bevy::{input::gamepad::GamepadEvent, prelude::*};

use super::marker::Creature;
use crate::components::chess_floor::{self, ChessFloorBundler};
use crate::shared::z_levels::ZLevel;
use crate::{
    components::{animation::AnimationController, movement::MovementController},
    features::creature::CreatureBundler,
};

pub fn debug_spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(CreatureBundler::player_bundle(
        asset_server.clone(),
        &mut texture_atlas_layouts,
    ));
}

pub fn debug_camera_seek_player(
    mut camera_query: Query<(&mut Transform, &Camera2d), (With<Camera2d>, Without<Creature>)>,
    player_transform: Single<&Transform, (With<Creature>, Without<Camera2d>)>,
) {
    for (mut camera_transform, camera) in &mut camera_query {
        camera_transform.translation = player_transform.translation.clone();
    }
}

pub fn record_creature_wasd_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Creature>>,
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
    mut query: Query<&mut AnimationController, With<Creature>>,
) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

pub fn update_creature_sprite_animation(
    mut query: Query<(&AnimationController, &mut Sprite), With<Creature>>,
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
    mut player_query: Query<
        (&MovementController, &mut Sprite, &mut AnimationController),
        With<Creature>,
    >,
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
