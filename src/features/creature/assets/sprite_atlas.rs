#![allow(dead_code)]

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Clone)]
pub struct CreatureSpriteAtlas {
    sprite: Sprite,
}
impl CreatureSpriteAtlas {
    pub fn new(asset_server: &AssetServer, image_path: &str, texture_atlas: TextureAtlas) -> Self {
        let sprite = Sprite {
            image: asset_server.load(image_path),
            texture_atlas: Some(texture_atlas),
            ..default()
        };

        Self { sprite }
    }

    #[inline]
    pub fn sprite(&self) -> Sprite {
        self.sprite.clone()
    }

    pub fn texture_atlas(&self) -> TextureAtlas {
        self.sprite.texture_atlas.clone().unwrap()
    }
}
