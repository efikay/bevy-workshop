#![allow(unused_imports)]

use bevy::prelude::*;

mod config;
mod systems;

use crate::{
    components::{_animovement, camera_targeting, health},
    features::projectile,
    shared::{AppSystems, PausableAppSystems},
};

pub use config::CreatureConfig;
pub mod markers;

pub fn plugin(app: &mut App) {
    app.add_plugins(_animovement::plugin);
    app.add_plugins(camera_targeting::plugin);
    app.add_plugins(projectile::plugin);
    app.add_plugins(health::plugin);

    app.add_systems(
        Update,
        (
            systems::record_player_action_input.in_set(AppSystems::RecordInput),
            systems::unpack_creature_configs.in_set(AppSystems::Update),
        )
            .in_set(PausableAppSystems),
    );

    #[cfg(feature = "inspector__creature")]
    {
        use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;

        app.add_plugins(FilterQueryInspectorPlugin::<With<markers::Creature>>::default());
        app.add_plugins(FilterQueryInspectorPlugin::<
            With<markers::creature_type::Player>,
        >::default());
    }
}
