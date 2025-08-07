use bevy::prelude::*;

mod component;
mod healthbar;

pub use component::{Health, TakeDamageResult};
pub use healthbar::healthbar_bundle;

pub fn plugin(app: &mut App) {
    app.add_plugins(healthbar::plugin);
}
