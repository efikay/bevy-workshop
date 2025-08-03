#![allow(dead_code)]

use bevy::prelude::*;

/// Basic movement component
/// 
/// Does not interact with other stuff except Bevy's Transform
#[derive(Component)]
pub struct MovementController {
    pub intent: Vec2,
    pub max_speed: f32,
}
impl MovementController {
    const DEFAULT_MAX_SPEED: f32 = 400.;
}
impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            max_speed: Self::DEFAULT_MAX_SPEED,
        }
    }
}
impl MovementController {
    pub fn new(max_speed: f32) -> Self {
        Self {
            max_speed,
            ..default()
        }
    }
}
