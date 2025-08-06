#![allow(unused_imports)]

use bevy::prelude::*;

mod config;
mod markers;
mod systems;

use crate::{
    components::{camera_targeting, _animovement},
    features::projectile,
    shared::{AppSystems, PausableAppSystems},
};

pub use config::CreatureConfig;

pub fn plugin(app: &mut App) {
    app.add_plugins(_animovement::plugin);
    app.add_plugins(camera_targeting::plugin);
    app.add_plugins(projectile::plugin);

    app.add_systems(
        Update,
        (
            (
                systems::record_creature_wasd_input,
                systems::record_creature_gamepad_movement_input,
                systems::record_player_action_input,
            )
                .in_set(AppSystems::RecordInput),
            systems::unpack_creature_configs.in_set(AppSystems::Update),
        )
            .in_set(PausableAppSystems),
    );

    #[cfg(feature = "inspector__creature")]
    {
        use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;

        app.add_plugins(FilterQueryInspectorPlugin::<With<markers::Creature>>::default());
    }
}
