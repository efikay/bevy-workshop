use std::ops::Range;

use bevy::image::TextureAtlasLayout;

use crate::{
    components::animation::AnimationState,
    shared::{data_structures::RangeDoubleMapper, direction::DirectionSimple},
};

pub struct CreatureConfig {
    pub atlas_sprite_path: String,
    pub atlas_layout: TextureAtlasLayout,
    pub atlas_grid_mapper: RangeDoubleMapper<AnimationState, DirectionSimple>,
    pub is_controlled: bool,
    pub is_camera_target: bool,
}
