mod screen_states;
mod gameplay_screen;

use bevy::prelude::*;

pub use screen_states::ScreenState;

pub fn plugin(app: &mut App) {
    app.init_state::<ScreenState>();

    app.add_plugins((
        gameplay_screen::plugin,
        // loading::plugin,
        // splash::plugin,
        // title::plugin,
    ));
}
