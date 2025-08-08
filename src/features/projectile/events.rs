use bevy::prelude::*;

/// ☄️
#[derive(Event, Reflect)]
pub struct SendProjectile {
    pub from: Transform,
    /// Must be non-zero
    pub intent: Vec2,
    pub speed: f32,
    pub damage: u32,
    // pub place_in: Option<Entity>,
    // pub despawn_after: Option<Duration>,
}
// Speed constants for debugging
impl SendProjectile {
    pub const VERY_SLOW: f32 = 10.;
    pub const SLOW: f32 = 30.;
    pub const ITS_OK: f32 = 100.;
    pub const BLAZINGLY_FAST: f32 = 1000.;
}
// Damage constants for debugging
impl SendProjectile {
    pub const D_MINOR: u32 = 20;
    pub const D_MAJOR: u32 = 100;
}

/// ☄️☄️☄️
/// ☄️🚁☄️
/// ☄️☄️☄️
#[derive(Event, Reflect)]
pub struct SendProjectilesAround {
    pub from: Transform,
    pub speed: f32,
    pub damage: u32,
    pub projectiles_amount: u8, // More than enough in order to keep GPU alive
}
