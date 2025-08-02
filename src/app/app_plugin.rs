use bevy::prelude::*;

use super::base_plugins;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(base_plugins::BasePlugins);

        println!("Hello! It's me, app!");
    }
}
