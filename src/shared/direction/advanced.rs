#![allow(dead_code)]

use std::f32;
use bevy::math::Vec2;

use super::utils::vec2_to_normalized_degrees;

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
    fn from(point: Vec2) -> Self {
        match vec2_to_normalized_degrees(point) {
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
            },
        }
    }
}
