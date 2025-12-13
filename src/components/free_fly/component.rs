use bevy::prelude::*;

#[derive(Component)]
#[require(Transform)]
pub enum FreeFly {
    // Capturing WASD-controls with linear speed
    WASDLinear(u32),
}
