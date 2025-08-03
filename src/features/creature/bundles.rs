#![allow(dead_code)]

use std::collections::HashMap;
use std::ops::Range;

use bevy::prelude::*;

use super::assets::CreatureAssetsBuilder;
use super::marker::Creature;
use crate::components::animation::{AnimationController, AnimationState};
use crate::components::movement::MovementController;
use crate::shared::z_levels::ZLevel;
use crate::shared::{data_structures::IterableRangeGrid, direction::DirectionSimple};

pub struct CreatureBundler;
impl CreatureBundler {
    // TODO: Move player stuff to constants?
    pub fn player_bundle(
        asset_server: AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    ) -> impl Bundle {
        use AnimationState as AnimState;

        let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(48, 64),
            8,
            12,
            None,
            None,
        ));

        CreatureBundler::bundle_simple_direction(
            asset_server,
            "sprites/character/idle+walk.png",
            TextureAtlas { layout, index: 0 },
            IterableRangeGrid::new(|animation_state| match animation_state {
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
        )
    }

    fn bundle_simple_direction(
        asset_server: AssetServer,
        atlas_image_path: &str,
        texture_atlas: TextureAtlas,
        frame_grid: IterableRangeGrid<AnimationState, DirectionSimple>,
    ) -> impl Bundle {
        let assets = {
            let mut builder = CreatureAssetsBuilder::new(asset_server);
            builder.with_atlas_image(&atlas_image_path, texture_atlas);

            builder.build().unwrap()
        };

        let sprite = assets.sprite_atlas.sprite();

        let animation = AnimationController::new(frame_grid);

        (
            Creature,
            sprite,
            animation,
            Transform::from_xyz(0.0, 0.0, ZLevel::Ground.into())
                .with_scale(Vec3::new(4.0, 4.0, 4.0)),
            MovementController::default(),
        )
    }
}
