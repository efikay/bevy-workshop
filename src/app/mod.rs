use bevy::prelude::*;

use crate::{
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

        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            scenes::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            ui_kit::plugin,
        ));

        println!("Hello! It's me, app!");
    }
}
