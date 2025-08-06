#![allow(dead_code)]

use bevy::math::Vec2;
use std::f32;

use crate::shared::utils::vec2::vec2_to_normalized_degrees;

use super::DirectionSimple;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default, Hash)]
pub enum DirectionAdvanced {
    North,
    NorthEast,
    East,
    SouthEast,
    #[default]
    South,
    SouthWest,
    West,
    NorthWest,
}
impl From<Vec2> for DirectionAdvanced {
    fn from(intent: Vec2) -> Self {
        match vec2_to_normalized_degrees(intent) {
            Some(degree) => match degree {
                a if a < 22.5 || a >= 337.5 => Self::East,
                a if a >= 22.5 && a < 67.5 => Self::NorthEast,
                a if a >= 67.5 && a < 112.5 => Self::North,
                a if a >= 112.5 && a < 157.5 => Self::NorthWest,
                a if a >= 157.5 && a < 202.5 => Self::West,
                a if a >= 202.5 && a < 247.5 => Self::SouthWest,
                a if a >= 247.5 && a < 292.5 => Self::South,
                a if a >= 292.5 && a < 337.5 => Self::SouthEast,
                _ => unreachable!(), // Angle is normalized to [0, 360)
            },
            None => {
                // Point is exactly in center, no direction. Let's stick with default
                Self::default()
            }
        }
    }
}

impl DirectionAdvanced {
    /// Not enough data so we "rotate+45deg" every mid-direction
    pub fn to_simple_lossy(&self) -> DirectionSimple {
        match self {
            DirectionAdvanced::North => DirectionSimple::North,
            DirectionAdvanced::NorthEast => DirectionSimple::East,
            DirectionAdvanced::East => DirectionSimple::East,
            DirectionAdvanced::SouthEast => DirectionSimple::South,
            DirectionAdvanced::South => DirectionSimple::South,
            DirectionAdvanced::SouthWest => DirectionSimple::West,
            DirectionAdvanced::West => DirectionSimple::West,
            DirectionAdvanced::NorthWest => DirectionSimple::North,
        }
    }
}
