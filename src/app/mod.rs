use bevy::prelude::*;

use crate::{
    features::game_cursor,
    menus, scenes, screens,
    shared::{asset_tracking, audio},
    ui_kit,
};

mod base_plugins;
#[cfg(feature = "dev")]
mod dev_tools;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(base_plugins::BasePlugins);

        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);

        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            scenes::plugin,
            menus::plugin,
            screens::plugin,
            ui_kit::plugin,
            game_cursor::plugin,
        ));

        println!("Hello! It's me, app!");
    }
}
