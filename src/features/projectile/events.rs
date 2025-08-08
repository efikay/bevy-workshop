use bevy::prelude::*;

#[derive(Event, Reflect)]
pub struct SendProjectile {
    pub from: Transform,
    /// Must be non-zero
    pub intent: Vec2,
    pub speed: f32,
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

pub mod debug {
    use super::*;

    #[derive(Event, Reflect)]
    pub struct RemoveAllProjectiles;
}
