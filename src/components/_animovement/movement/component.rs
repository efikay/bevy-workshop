#![allow(dead_code)]

use bevy::prelude::*;

/// Basic movement component
/// 
/// Does not interact with other stuff except Bevy's Transform
#[derive(Component, Reflect)]
pub struct Movement {
    pub intent: Vec2,
    pub max_speed: f32,
}
impl Movement {
    const DEFAULT_MAX_SPEED: f32 = 400.;
}
impl Default for Movement {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            max_speed: Self::DEFAULT_MAX_SPEED,
        }
    }
}
impl Movement {
    pub fn new(max_speed: f32) -> Self {
        Self {
            max_speed,
            ..default()
        }
    }
}
