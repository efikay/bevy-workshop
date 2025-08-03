#![allow(dead_code)]

use bevy::prelude::*;

use super::sprite_atlas::CreatureSpriteAtlas;

#[derive(Component)]
pub struct CreatureAssets {
    pub sprite_atlas: CreatureSpriteAtlas,
    pub step_sounds: Option<Vec<Handle<AudioSource>>>,
}
impl CreatureAssets {
    pub fn new(
        sprite_atlas: CreatureSpriteAtlas,
        step_sounds: Option<Vec<Handle<AudioSource>>>,
    ) -> Self {
        Self {
            sprite_atlas,
            step_sounds,
        }
    }
}
