use bevy::prelude::Vec2;

pub fn is_negative_direction(vec: Vec2) -> bool {
    vec.x < 0.0 || vec.y < 0.0
}
