use bevy::prelude::*;

pub mod player_scene;

pub fn plugin(app: &mut App) {
    app.add_plugins(player_scene::plugin);
}

/**
 * This module is used only in pair with *gameplay_screen* in mind
 */
fn _doc() {}
