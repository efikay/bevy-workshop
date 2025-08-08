use bevy::prelude::*;

// Marker struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
pub struct FpsCounterValueText;

// Marker struct to help identify the color-changing Text component
#[derive(Component)]
pub struct FpsCounterText;
