pub mod marker;
mod systems;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, systems::camera_follow_target);
}
