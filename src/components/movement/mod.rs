use bevy::prelude::*;

mod controller;
mod plugin;
mod system;

pub use controller::MovementController;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, system::apply_movement);
    }
}
