#![allow(unused_imports)]

use bevy::prelude::*;

mod config;
mod markers;
mod systems;

use crate::{
    components::{camera_targeting, movement},
    shared::{AppSystems, PausableAppSystems},
};

pub use config::CreatureConfig;

pub fn plugin(app: &mut App) {
    app.add_plugins(movement::plugin);
    app.add_plugins(camera_targeting::plugin);

    app.add_systems(
        Update,
        (
            systems::tick_creature_animation_timer.in_set(AppSystems::TickTimers),
            (
                systems::record_creature_wasd_input,
                systems::record_creature_gamepad_movement_input,
            )
                .in_set(AppSystems::RecordInput),
            (
                systems::update_creature_animation_state,
                systems::update_creature_sprite_animation,
                systems::unpack_creature_configs,
            )
                .chain()
                .in_set(AppSystems::Update),
        )
            .in_set(PausableAppSystems),
    );

    #[cfg(feature = "inspector__creature")]
    {
        use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;

        app.add_plugins(FilterQueryInspectorPlugin::<With<markers::Creature>>::default());
    }
}
