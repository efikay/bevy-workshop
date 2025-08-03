#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Default, Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum AnimationState {
    #[default]
    Idle,
    Walk,
}
impl From<Vec2> for AnimationState {
    fn from(point: Vec2) -> Self {
        match point == Vec2::ZERO {
            true => Self::Idle,
            false => Self::Walk,
        }
    }
}
