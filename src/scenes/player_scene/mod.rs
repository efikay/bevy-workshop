//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

use crate::{components::screen_wrap, features::creature};

pub mod level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((screen_wrap::plugin, creature::plugin, level::plugin));

    // app.add_plugins(audio_mixer::plugin);
}
