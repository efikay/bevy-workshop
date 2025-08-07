use bevy::prelude::*;

use crate::shared::{AppSystems, PausableAppSystems};

mod animation;
mod movement;

mod systems;

pub use animation::{AnimationState, Animation};
pub use movement::Movement;

pub fn plugin(app: &mut App) {
    app.register_type::<Movement>();

    app.add_systems(
        Update,
        (
            animation::systems::tick_animation_timer.in_set(AppSystems::TickTimers),
            (
                movement::system::apply_movement,
                systems::update_animation_from_movement,
                animation::systems::update_sprite_animation,
            )
                .chain()
                .in_set(AppSystems::Update),
        ).in_set(PausableAppSystems),
    );
}
