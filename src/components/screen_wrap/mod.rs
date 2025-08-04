use bevy::prelude::*;

use crate::shared::{AppSystems, PausableAppSystems};

mod component;
mod system;

pub use component::ScreenWrap;

pub fn plugin(app: &mut App) {
    app.register_type::<component::ScreenWrap>();

    app.add_systems(
        Update,
        (system::apply_screen_wrap)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableAppSystems),
    );
}
