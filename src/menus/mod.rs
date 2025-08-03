//! The game's menus and transitions between them.

mod main;
mod menu_states;
mod pause;
mod settings;

use bevy::prelude::*;
pub use menu_states::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<MenuState>();

    app.add_plugins((main::plugin, settings::plugin, pause::plugin));
}
