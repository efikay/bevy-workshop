use bevy::prelude::*;

#[derive(Component)]
pub struct Healthbar;

impl Healthbar {
    pub const HEALTH_COLOR: Color = Color::srgb_u8(0, 200, 55);

    pub const MAX_WIDTH: f32 = 100.0;
    pub const HEIGHT: f32 = 10.0;
}
