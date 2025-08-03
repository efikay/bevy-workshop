use bevy::prelude::*;

use super::marker::FpsLimiterText;

pub fn bundle() -> impl Bundle {
    (
        Text::new("[F]PS Limit: "),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(30.),
            ..default()
        },
        children![(TextSpan::default(), FpsLimiterText)],
    )
}
