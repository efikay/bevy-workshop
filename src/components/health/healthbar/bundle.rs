use bevy::prelude::*;

use super::*;

pub fn healthbar_bundle() -> impl Bundle {
    (
        marker::Healthbar,
        Transform::from_xyz(0.0, 0.0, 2.0),
        Sprite {
            color: marker::Healthbar::HEALTH_COLOR,
            custom_size: Some(Vec2::new(
                marker::Healthbar::MAX_WIDTH,
                marker::Healthbar::HEIGHT,
            )),
            ..default()
        },
    )
}
