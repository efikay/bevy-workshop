//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
};

use crate::{screens::ScreenState, shared::AppPauseState};

const TOGGLE_KEY: KeyCode = KeyCode::KeyZ;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, log_transitions::<ScreenState>);
    app.add_systems(Update, log_transitions::<AppPauseState>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
