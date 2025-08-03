use bevy::prelude::*;

mod component;
mod system;

pub use component::Movement;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, system::apply_movement);
    }
}
