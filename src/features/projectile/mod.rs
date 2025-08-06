use bevy::prelude::*;

pub mod events;
mod marker;
mod systems;

pub fn plugin(app: &mut App) {
    app.add_event::<events::SendProjectile>();
    app.register_type::<events::SendProjectile>();

    app.add_systems(Update, systems::spawn_event_projectiles);
}
