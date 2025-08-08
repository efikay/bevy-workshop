use bevy::prelude::*;

pub mod events;
mod component;
mod systems;
pub mod observer;

/**
 * ⚠️ This module uses creature (nearby module)
 */
fn _doc() {}

pub fn plugin(app: &mut App) {
    app.add_event::<events::SendProjectile>();
    app.register_type::<events::SendProjectile>();

    app.add_event::<events::SendProjectilesAround>();
    app.register_type::<events::SendProjectilesAround>();

    app.add_systems(Update, systems::spawn_event_projectiles);
    app.add_systems(Update, systems::spawn_projectiles_around_from_event);
}
