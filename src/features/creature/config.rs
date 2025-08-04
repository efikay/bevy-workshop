use std::ops::Range;

use bevy::image::TextureAtlasLayout;
use bevy::prelude::*;

use crate::{
    components::animation::AnimationState,
    shared::{data_structures::RangeDoubleMapper, direction::DirectionSimple, z_levels::ZLevel},
};

pub struct CreatureConfig {
    pub atlas_sprite_path: String,
    pub atlas_layout: TextureAtlasLayout,
    pub atlas_grid_mapper: RangeDoubleMapper<AnimationState, DirectionSimple>,
    pub is_controlled: bool,
    pub is_camera_target: bool,
    pub initial_transform: Transform,
}

impl CreatureConfig {
    pub fn player() -> Self {
        use AnimationState as AnimState;

        Self {
            atlas_sprite_path: String::from("sprites/character/male/idle+walk.png"),
            atlas_layout: TextureAtlasLayout::from_grid(UVec2::new(48, 64), 8, 12, None, None),
            atlas_grid_mapper: RangeDoubleMapper::new(|animation_state| match animation_state {
                AnimState::Walk => |direction| match direction {
                    DirectionSimple::North => 24..32,
                    DirectionSimple::South => 0..8,
                    DirectionSimple::East => 40..48,
                    DirectionSimple::West => 8..16,
                },
                AnimState::Idle => |direction| match direction {
                    DirectionSimple::North => 72..80,
                    DirectionSimple::South => 48..56,
                    DirectionSimple::East => 88..96,
                    DirectionSimple::West => 56..64,
                },
            }),
            is_controlled: true,
            is_camera_target: true,
            initial_transform: Transform::from_xyz(0.0, 0.0, ZLevel::Ground.into())
                .with_scale(Vec3::new(4.0, 4.0, 4.0)),
        }
    }

    pub fn npc(initial_position: Vec2) -> Self {
        use AnimationState as AnimState;

        let cols = 8;
        let rows = 12;

        Self {
            atlas_sprite_path: String::from("sprites/character/female/idle+walk.png"),
            atlas_layout: TextureAtlasLayout::from_grid(UVec2::new(48, 64), cols, rows, None, None),
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
            is_controlled: false,
            is_camera_target: false,
            initial_transform: Transform::from_translation(
                initial_position.extend(ZLevel::Ground.into()),
            )
            .with_scale(Vec3::new(4.0, 4.0, 4.0)),
        }
    }
}
