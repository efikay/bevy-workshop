use bevy::prelude::*;

pub mod player_scene;

pub fn plugin(app: &mut App) {
    app.add_plugins(player_scene::plugin);
}