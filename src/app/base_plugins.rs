use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::{CursorOptions, WindowMode},
};

use crate::shared::{AppPauseState, AppSystems, PausableAppSystems};

// Base plugin collection. Used to configure default plugins and prepare the game
pub struct BasePlugins;
impl Plugin for BasePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(Self::asset_plugin())
                .set(Self::window_plugin()),
        );

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(Update, AppSystems::system_set().chain());

        // Set up the `Pause` state.
        app.init_state::<AppPauseState>();
        app.init_state::<AppPauseState>();
        app.configure_sets(
            Update,
            PausableAppSystems.run_if(in_state(AppPauseState(false))),
        );

        app.add_systems(Startup, Self::spawn_camera);

        Self::set_clear_color(app);
    }
}

impl BasePlugins {
    const CLEAR_COLOR: Color = Color::srgb_u8(52, 29, 90);

    fn set_clear_color(app: &mut App) {
        app.insert_resource(ClearColor(Self::CLEAR_COLOR));
    }

    fn spawn_camera(mut commands: Commands) {
        commands.spawn(Camera2d);
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
