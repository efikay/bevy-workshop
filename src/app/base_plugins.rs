use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::{CursorOptions, WindowMode},
};
use bevy_rapier2d::{prelude::*, rapier::prelude::IntegrationParameters};

use crate::shared::{AppPauseState, AppSystems, PausableAppSystems};

// Base plugin collection. Used to configure default plugins and prepare the game
pub struct BasePlugins;
impl Plugin for BasePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(Self::asset_plugin())
                .set(ImagePlugin::default_nearest())
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

        Self::attach_rapier2d_physics_plugin(app);
        Self::set_clear_color(app);

        // #[cfg(feature = "dev")]
        // {
        //     app.add_systems(Startup, debug::setup_physics);
        //     app.add_systems(Update, debug::print_ball_altitude);
        // }
    }
}

impl BasePlugins {
    const CLEAR_COLOR: Color = Color::srgb_u8(52, 29, 90);

    fn set_clear_color(app: &mut App) {
        app.insert_resource(ClearColor(Self::CLEAR_COLOR));
    }

    fn attach_rapier2d_physics_plugin(app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::with_custom_initialization(
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierContextInitialization::InitializeDefaultRapierContext {
                integration_parameters: IntegrationParameters::default(),
                rapier_configuration: RapierConfiguration {
                    gravity: Vec2::ZERO,
                    physics_pipeline_active: true,
                    scaled_shape_subdivision: 10,
                    force_update_from_transform_changes: false,
                },
            },
        ));
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

mod debug {
    #![allow(dead_code)]

    use super::*;

    pub fn setup_physics(mut commands: Commands) {
        /* Create the ground. */
        commands
            .spawn(Collider::cuboid(500.0, 50.0))
            .insert(Transform::from_xyz(0.0, -100.0, 0.0));

        /* Create the bouncing ball. */
        commands
            .spawn(RigidBody::Dynamic)
            .insert(Collider::ball(50.0))
            .insert(Restitution::coefficient(0.7))
            .insert(Transform::from_xyz(0.0, 400.0, 0.0));
    }

    pub fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
        for transform in positions.iter() {
            println!("Ball altitude: {}", transform.translation.y);
        }
    }
}
