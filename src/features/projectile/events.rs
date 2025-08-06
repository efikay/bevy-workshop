use bevy::prelude::*;

#[derive(Event, Reflect)]
pub struct SendProjectile {
    pub from: Transform,
    pub speed: f32,
    // pub place_in: Option<Entity>,
    // pub despawn_after: Option<Duration>,
}
