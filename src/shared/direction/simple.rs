use bevy::math::Vec2;

use lib::utils::vec2::vec2_to_normalized_degrees;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default, Hash)]
pub enum DirectionSimple {
    North,
    East,
    #[default]
    South,
    West,
}
impl From<Vec2> for DirectionSimple {
    fn from(value: Vec2) -> Self {
        if let Some(degree) = vec2_to_normalized_degrees(value) {
            if degree >= 315.0 && degree < 360.0 || (degree >= 0.0 && degree < 45.0) {
                DirectionSimple::East
            } else if degree >= 45.0 && degree < 135.0 {
                DirectionSimple::North
            } else if degree >= 135.0 && degree < 225.0 {
                DirectionSimple::West
            } else {
                DirectionSimple::South
            }
        } else {
            Self::default()
        }
    }
}

impl DirectionSimple {
    pub fn to_max_intent(&self) -> Vec2 {
        match self {
            DirectionSimple::North => Vec2::new(0.0, 1.0),
            DirectionSimple::East => Vec2::new(1.0, 0.0),
            DirectionSimple::South => Vec2::new(0.0, -1.0),
            DirectionSimple::West => Vec2::new(-1.0, 0.0),
        }
    }
}
