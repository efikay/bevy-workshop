#![allow(dead_code)]

use std::collections::HashMap;
use std::ops::Range;

use bevy::ecs::entity_disabling::Disabled;
use bevy::prelude::*;

use super::assets::CreatureAssetsBuilder;
use super::markers::Creature;
use crate::components::animation::{Animation, AnimationState};
use crate::components::camera_targeting::marker::CameraTarget;
use crate::components::movement::Movement;
use crate::shared::z_levels::ZLevel;
use crate::shared::{data_structures::RangeDoubleMapper, direction::DirectionSimple};

use super::config;

fn player_cfg() -> config::CreatureConfig {
    use AnimationState as AnimState;

    config::CreatureConfig {
        atlas_sprite_path: String::from("sprites/character/idle+walk.png"),
        atlas_layout: TextureAtlasLayout::from_grid(UVec2::new(48, 64), 8, 12, None, None),
        atlas_grid_mapper: RangeDoubleMapper::new(|animation_state| match animation_state {
            AnimState::Idle => |direction| match direction {
                DirectionSimple::North => 24..32,
                DirectionSimple::South => 0..8,
                DirectionSimple::East => 40..48,
                DirectionSimple::West => 8..16,
            },
            AnimState::Walk => |direction| match direction {
                DirectionSimple::North => 72..80,
                DirectionSimple::South => 48..56,
                DirectionSimple::East => 88..96,
                DirectionSimple::West => 56..64,
            },
        }),
        is_controlled: true,
        is_camera_target: true,
    }
}

pub fn player_bundle(
    asset_server: AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    bundle(asset_server, texture_atlas_layouts, player_cfg())
}

fn bundle(
    asset_server: AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    config: config::CreatureConfig,
) -> impl Bundle {
    use AnimationState as AnimState;

    let layout = texture_atlas_layouts.add(config.atlas_layout);

    let assets = {
        let mut builder = CreatureAssetsBuilder::new(asset_server);
        builder.with_atlas_image(&config.atlas_sprite_path, TextureAtlas { layout, index: 0 });

        builder.build().unwrap()
    };

    let sprite = assets.sprite_atlas.sprite();

    let animation = Animation::new(config.atlas_grid_mapper);

    (
        Creature {
            is_controlled: config.is_controlled,
        },
        CameraTarget {
            is_enabled: config.is_camera_target,
        },
        sprite,
        animation,
        Transform::from_xyz(0.0, 0.0, ZLevel::Ground.into()).with_scale(Vec3::new(4.0, 4.0, 4.0)),
        Movement::default(),
    )
}
