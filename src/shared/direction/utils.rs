use std::f32;
use bevy::math::Vec2;

/// Convert vec2 which represents point to normal degree
/// 
/// Returns None in case point is exactly on center (no direction in that case)
pub fn vec2_to_normalized_degrees(point: Vec2) -> Option<f32> {
    if point.length() == 0. {
        None
    } else {
        let (x, y) = point.into();

        // Calculate angle in radians
        let angle = y.atan2(x); // range: [-π, π]

        // Normalize angle to [0, 2π)
        let normalized_angle = if angle < 0.0 {
            angle + 2.0 * f32::consts::PI
        } else {
            angle
        };

        Some(normalized_angle.to_degrees())
    }
}
