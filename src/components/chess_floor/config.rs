use bevy::prelude::*;

pub struct ChessFloorConfig {
    pub area: Rect,
    pub tile_size: f32,
    pub black_tile_color: Color,
    pub white_tile_color: Color,
}

impl Default for ChessFloorConfig {
    fn default() -> Self {
        Self {
            area: Rect::default(),
            tile_size: 40.,
            black_tile_color: Color::srgb_u8(89, 89, 89),
            white_tile_color: Color::srgb_u8(166, 166, 166),
        }
    }
}
