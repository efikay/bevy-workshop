use bevy::prelude::*;

mod bundle;
mod marker;
mod systems;

pub use bundle::healthbar_bundle;

use crate::shared::AppSystems;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, systems::update_healthbar.in_set(AppSystems::Update));
}
