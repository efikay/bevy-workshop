use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::{CursorOptions, WindowMode},
};

use crate::shared::AppSystems;

// Base plugin collection. Used to configure default plugins and prepare the game
pub struct BasePlugins;
impl Plugin for BasePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(Self::asset_plugin())
                .set(Self::window_plugin()),
        );

        app.configure_sets(Update, AppSystems::system_set());

        Self::spawn_camera(app);
        Self::set_clear_color(app, Color::srgb_u8(52, 29, 90));
    }
}

impl BasePlugins {
    fn set_clear_color(app: &mut App, color: Color) {
        app.insert_resource(ClearColor(color));
    }

    fn spawn_camera(app: &mut App) {
        let world = app.world_mut();

        world.commands().spawn(Camera2d);
    }

    fn asset_plugin() -> AssetPlugin {
        AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }
    }

    fn window_plugin() -> WindowPlugin {
        WindowPlugin {
            primary_window: Window {
                title: "2d Playground".into(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Index(1)),
                cursor_options: CursorOptions { ..default() },
                fit_canvas_to_parent: true,
                ..default()
            }
            .into(),
            ..default()
        }
    }
}
