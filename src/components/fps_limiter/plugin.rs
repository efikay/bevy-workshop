use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_framepace::FramepacePlugin;

use super::{resource::FpsLimiter, systems::FpsLimiterSystems};

pub struct FpsLimiterPlugin;
impl Plugin for FpsLimiterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FramepacePlugin);

        app.init_resource::<FpsLimiter>();

        app.add_systems(
            Startup,
            (FpsLimiterSystems::setup, FpsLimiterSystems::update_text).chain(),
        );
        app.add_systems(
            Update,
            ((
                FpsLimiterSystems::next_fps_limit,
                FpsLimiterSystems::update_text,
            )
                .chain()
                .run_if(input_just_pressed(KeyCode::KeyF)),),
        );
    }
}
