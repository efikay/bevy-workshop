//! The game's menus and transitions between them.
use bevy::prelude::*;

mod main;
mod pause;
mod settings;

pub mod state;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<state::MenuState>();

    app.add_plugins((main::plugin, settings::plugin, pause::plugin));
}
