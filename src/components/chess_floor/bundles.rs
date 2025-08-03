use bevy::{prelude::*, sprite::Anchor};

use crate::{
    components::chess_floor::config::ChessFloorConfig,
    shared::{
        data_structures::{ChunkToGrid as _, PrimitiveRect},
        z_levels::ZLevel,
    },
};

pub fn make_sprite_bundles(config: ChessFloorConfig) -> Vec<(Sprite, Transform)> {
    let ChessFloorConfig {
        tile_size,
        black_tile_color,
        white_tile_color,
        area,
    } = config;

    // it's definitely possible to fix it in this fn below
    let tile_areas =
        PrimitiveRect::from(area).chunk_to_grid_with_cuttings(Vec2::new(tile_size, tile_size));

    let mut children = vec![];

    for ((row_idx, col_idx), area) in tile_areas.indexed_iter() {
        let is_black = (row_idx + col_idx) % 2 == 0;

        let xy: Vec2 = area.min();
        let (width, height) = (area.max() - xy).into();

        children.push((
            Sprite {
                color: if is_black {
                    black_tile_color
                } else {
                    white_tile_color
                },
                // FIXME: looks not quite good with ROW cuttings when anchor set to TopLeft (bug or feature?)
                anchor: Anchor::BottomLeft,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            Transform::from_translation(Vec3 {
                x: xy.x,
                y: xy.y,
                z: ZLevel::Floor.into(),
            }),
        ));
    }

    children
}
