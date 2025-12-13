// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev_tools"), windows_subsystem = "windows")]

use bevy::prelude::*;

mod app;
mod menus;
mod screens;
mod shared;
mod ui_kit;
mod scenes;
mod features;
mod components;

fn main() -> AppExit {
    App::new().add_plugins(app::AppPlugin).run()
}
