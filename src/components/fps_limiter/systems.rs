use bevy::prelude::*;
use bevy_framepace::FramepaceSettings;

use super::{bundler::FpsLimiterBundler, marker::FpsLimiterText, resource::FpsLimiter};

pub struct FpsLimiterSystems;
impl FpsLimiterSystems {
    pub fn setup(mut commands: Commands) {
        commands.spawn(FpsLimiterBundler::text());
    }

    pub fn update_text(
        settings: Res<FramepaceSettings>,
        mut text: Single<&mut TextSpan, With<FpsLimiterText>>,
    ) {
        text.0 = settings.limiter.to_string();
    }

    pub fn next_fps_limit(
        mut settings: ResMut<FramepaceSettings>,
        mut limiter: ResMut<FpsLimiter>,
    ) {
        settings.limiter = limiter.to_next();
    }
}
