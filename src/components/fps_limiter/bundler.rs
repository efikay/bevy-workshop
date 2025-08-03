use bevy::prelude::*;

use super::marker::FpsLimiterText;

pub struct FpsLimiterBundler;
impl FpsLimiterBundler {
    pub fn text() -> impl Bundle {
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
}
