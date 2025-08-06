use bevy::prelude::*;

#[derive(Event, Reflect)]
pub struct SendProjectile {
    pub from: Transform,
    pub intent: Vec2,
    pub speed: f32,
    // pub place_in: Option<Entity>,
    // pub despawn_after: Option<Duration>,
}
