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
use crate::shared::data_structures::{ChunkToGrid, PrimitiveRect};
use crate::shared::z_levels::ZLevel;
use crate::shared::{data_structures::RangeDoubleMapper, direction::DirectionSimple};

use super::config;

type CreatureBundle = (
    Creature,
    CameraTarget,
    Sprite,
    Animation,
    Transform,
    Movement,
);

pub fn player_bundle(
    asset_server: AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    bundle(
        asset_server,
        texture_atlas_layouts,
        config::CreatureConfig::player(),
    )
}

pub fn npc_bundle(
    asset_server: AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    position: Vec2,
) -> impl Bundle {
    bundle(
        asset_server,
        texture_atlas_layouts,
        config::CreatureConfig::npc(position),
    )
}

fn bundle(
    asset_server: AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    config: config::CreatureConfig,
) -> CreatureBundle {
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
        config.initial_transform,
        Movement::default(),
    )
}
