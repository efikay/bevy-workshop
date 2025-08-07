#![allow(dead_code)]

use bevy::input::gamepad::GamepadConnection;
use bevy::{input::gamepad::GamepadEvent, prelude::*};

use super::markers::{
    Creature,
    creature_type::{EnemyNPC, FriendNPC, NeutralNPC, Player},
};
use crate::components::_animovement::{Animation, Movement};
use crate::components::camera_targeting::marker::CameraTarget;
use crate::features;
use crate::features::creature::config;
use crate::features::projectile::events::SendProjectile;
use crate::shared::common_markers::WASD;
use crate::shared::data_structures::{ChunkToGrid, PrimitiveRect};
use crate::shared::z_levels::ZLevel;

/// Searches for bundles with [`CreatureConfig`]-s and "unpacks" them into
/// full creature bundle with conditional markers on top
///
/// TODO: Probably not the best approach (it surely consumes some extra resources (by filling the scheduler at least))
/// Also it triggers not instantly AFAIK
///
/// TODO: better dynamic bundle approaches?
pub fn unpack_creature_configs(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    configs_query: Query<(&config::CreatureConfig, Entity), With<config::CreatureConfig>>,
    mut commands: Commands,
) {
    for (config, entity) in configs_query {
        commands
            .entity(entity)
            .remove::<config::CreatureConfig>()
            .insert((
                Creature,
                Movement::default(),
                Sprite {
                    image: asset_server.load(config.atlas_sprite_path.clone()),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layouts.add(config.atlas_layout.clone()),
                        index: 0,
                    }),
                    ..default()
                },
                Animation::new(config.atlas_grid_mapper.clone()),
                config.initial_transform,
            ))
            .insert_if(CameraTarget, || config.is_camera_target)
            .insert_if(WASD, || config.is_controlled)
            .insert_if(Player, || {
                config.creature_type == config::CreatureType::Player
            })
            .insert_if(EnemyNPC, || {
                config.creature_type == config::CreatureType::EnemyNPC
            })
            .insert_if(FriendNPC, || {
                config.creature_type == config::CreatureType::FriendNPC
            })
            .insert_if(NeutralNPC, || {
                config.creature_type == config::CreatureType::NeutralNPC
            });
    }
}

pub fn record_creature_wasd_input(
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

pub fn record_player_action_input(
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&Transform, &Movement, &Animation), With<Player>>,
    mut writer: EventWriter<SendProjectile>,
) {
    let transform = player.0;
    let movement = player.1;
    let animation = player.2;

    if input.just_pressed(KeyCode::KeyE) {
        let intent = match animation.idle_direction() {
            Some(direction) => {
                direction.to_max_intent()
            },
            // We're still moving then
            None => movement.intent,
        };

        writer.write(SendProjectile {
            intent,
            from: transform.clone(),
            speed: SendProjectile::BLAZINGLY_FAST,
        });
    }
}

pub fn record_creature_gamepad_movement_input(
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
