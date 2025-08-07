use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

mod bundle;
mod marker;
mod systems;

pub use bundle::healthbar_bundle;

use crate::shared::AppSystems;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, systems::update_healthbar.in_set(AppSystems::Update));

    // app.add_plugins(debug::plugin);
}

mod debug {
    use super::*;

    pub fn plugin(app: &mut App) {
        app.add_systems(
            Update,
            systems::debug::take_damage_if_alive
                .in_set(AppSystems::TickTimers)
                .run_if(on_timer(Duration::from_secs(1))),
        );
    }
}
