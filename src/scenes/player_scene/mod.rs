use bevy::prelude::*;

use crate::{components::screen_wrap, features::creature};

pub mod level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((screen_wrap::plugin, creature::plugin, level::plugin));
    // app.add_plugins(audio_mixer::plugin);
}
