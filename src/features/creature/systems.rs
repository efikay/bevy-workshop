#![allow(dead_code)]

use avian2d::prelude::*;
use bevy::input::gamepad::GamepadConnection;
use bevy::{input::gamepad::GamepadEvent, prelude::*};

use super::markers::{
    Creature,
    creature_type::{EnemyNPC, FriendNPC, NeutralNPC, Player},
};
use crate::components::_animovement::{Animation, Movement};
use crate::components::camera_targeting::marker::CameraTarget;
use crate::components::health::{Health, healthbar_bundle};
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
        let mut entity_commands = commands.entity(entity);
        entity_commands
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
                Health::new(config.hp),
                Animation::new(config.atlas_grid_mapper.clone()),
                config.initial_transform,
                children![healthbar_bundle()],
            ))
            .insert((RigidBody::Static, Collider::rectangle(16.0, 30.0)))
            .insert_if(CameraTarget, || config.is_camera_target)
            .insert_if(WASD, || config.is_controlled);

        match config.creature_type {
            config::CreatureType::Player => entity_commands.insert(Player),
            config::CreatureType::EnemyNPC => entity_commands.insert(EnemyNPC),
            config::CreatureType::FriendNPC => entity_commands.insert(FriendNPC),
            config::CreatureType::NeutralNPC => entity_commands.insert(NeutralNPC),
        };
    }
}

pub fn record_player_action_input(
    kbd: Res<ButtonInput<KeyCode>>,
    gamepad: Query<&Gamepad>,
    player: Single<(&Transform, &Movement, &Animation), With<Player>>,
    mut writer: EventWriter<SendProjectile>,
) {
    let transform = player.0;
    let movement = player.1;
    let animation = player.2;

    if kbd.just_pressed(KeyCode::KeyE)
        || gamepad
            .iter()
            .next()
            .is_some_and(|g| g.pressed(GamepadButton::South))
    {
        let intent = match animation.idle_direction() {
            Some(direction) => direction.to_max_intent(),
            // We're still moving then
            None => movement.intent,
        };

        writer.write(SendProjectile {
            intent,
            from: transform.clone(),
            speed: SendProjectile::ITS_OK,
        });
    }
}
