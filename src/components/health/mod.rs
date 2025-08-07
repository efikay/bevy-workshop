use bevy::prelude::*;

mod component;
mod healthbar;

pub use component::{Health, TakeDamageResult};
pub use healthbar::healthbar_bundle;

pub fn plugin(app: &mut App) {
    app.add_plugins(healthbar::plugin);

    #[cfg(feature = "inspector__health")]
    debug::set_up_inspector(app);
}

mod debug {
    use super::*;

    #[cfg(feature = "inspector__health")]
    pub fn set_up_inspector(app: &mut App) {
        use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;

        app.add_plugins(FilterQueryInspectorPlugin::<With<Health>>::default());
    }
}