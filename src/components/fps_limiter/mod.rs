#![allow(unused_imports)]

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_framepace::FramepacePlugin;

mod bundle;
mod config;
mod marker;
mod resource;
mod systems;

pub use marker::FpsLimiterText;

pub fn plugin(app: &mut App) {
    app.add_plugins(FramepacePlugin);

    app.init_resource::<resource::FpsLimiter>();

    app.add_systems(Startup, (systems::setup, systems::update_text).chain());
    app.add_systems(
        Update,
        ((systems::next_fps_limit, systems::update_text)
            .chain()
            .run_if(input_just_pressed(KeyCode::KeyF)),),
    );
}
