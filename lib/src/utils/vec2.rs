use bevy::prelude::Vec2;

pub fn is_negative_direction(vec: Vec2) -> bool {
    vec.x < 0.0 || vec.y < 0.0
}

pub fn vec2_to_normalized_degrees(point: Vec2) -> Option<f32> {
    if point.length() == 0. {
        None
    } else {
        let (x, y) = point.into();

        // Calculate angle in radians
        let angle = y.atan2(x); // range: [-π, π]

        // Normalize angle to [0, 2π)
        let normalized_angle = if angle < 0.0 {
            angle + 2.0 * std::f32::consts::PI
        } else {
            angle
        };

        Some(normalized_angle.to_degrees())
    }
}
