#![allow(dead_code)]

use bevy::prelude::*;

use super::sprite_atlas::CreatureSpriteAtlas;
use super::component::CreatureAssets;

#[derive(Component)]
pub struct CreatureAssetsBuilder {
    asset_server: AssetServer,

    sprite_atlas: Option<CreatureSpriteAtlas>,
    step_sounds: Option<Vec<Handle<AudioSource>>>,
}
impl CreatureAssetsBuilder {
    pub fn new(asset_server: AssetServer) -> Self {
        Self {
            asset_server,
            sprite_atlas: None,
            step_sounds: None,
        }
    }

    pub fn with_atlas_image(
        &mut self,
        image_path: &str,
        texture_atlas: TextureAtlas,
    ) -> &Self {
        self.sprite_atlas = Some(CreatureSpriteAtlas::new(
            &self.asset_server,
            image_path,
            texture_atlas,
        ));

        self
    }

    pub fn with_step_sounds(&mut self, sounds: Vec<&str>) -> &Self {
        self.step_sounds = Some(
            sounds
                .into_iter()
                .map(|sound| self.asset_server.load(sound))
                .collect(),
        );

        self
    }

    pub fn build(&mut self) -> Option<CreatureAssets> {
        let sprite_atlas = std::mem::replace(&mut self.sprite_atlas, None);
        let step_sounds = std::mem::replace(&mut self.step_sounds, None);

        match (sprite_atlas, step_sounds) {
            (Some(atlas_image), step_sounds) => Some(CreatureAssets::new(atlas_image, step_sounds)),
            _ => None,
        }
    }
}
