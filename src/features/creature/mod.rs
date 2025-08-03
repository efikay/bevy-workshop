#![allow(unused_imports)]

use bevy::prelude::*;

mod assets;
mod bundles;
mod marker;
mod systems;

pub use bundles::CreatureBundler;

use crate::components::movement::MovementPlugin;

/**
 * This module uses:
 *  - bits: animation, movement
 */
fn _doc() {}

pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MovementPlugin);

        app.add_systems(FixedUpdate, systems::record_creature_wasd_input);
        app.add_systems(FixedUpdate, systems::tick_creature_animation_timer);
        app.add_systems(
            FixedUpdate,
            (
                systems::update_creature_animation_state,
                systems::update_creature_sprite_animation,
            ),
        );

        app.add_systems(Startup, systems::debug_spawn_player);
        app.add_systems(Update, systems::debug_camera_seek_player);
    }
}
