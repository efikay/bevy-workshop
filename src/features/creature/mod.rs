#![allow(unused_imports)]

use bevy::prelude::*;

mod assets;
mod bundles;
mod config;
mod markers;
mod systems;

use crate::{
    components::{camera_targeting, movement},
    shared::{AppSystems, PausableAppSystems},
};
pub use bundles::player_bundle;

pub fn plugin(app: &mut App) {
    app.add_plugins(movement::plugin);
    app.add_plugins(camera_targeting::plugin);

    app.add_systems(
        Update,
        (
            systems::tick_creature_animation_timer.in_set(AppSystems::TickTimers),
            systems::record_creature_wasd_input.in_set(AppSystems::RecordInput),
            (
                systems::update_creature_animation_state,
                systems::update_creature_sprite_animation,
            )
                .chain()
                .in_set(AppSystems::Update),
        )
            .in_set(PausableAppSystems),
    );
}
