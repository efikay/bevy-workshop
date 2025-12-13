use bevy::prelude::*;

use crate::{components::chess_floor, features::creature, screens::ScreenState};
use lib::data_structures::rect_ext::{ChunkToGrid, RectExt};

pub(super) fn plugin(_: &mut App) {
    //
}

pub fn spawn_level(mut commands: Commands) {
    let spawn_zone = Rect::from_corners(Vec2::new(-1000.0, -1000.0), Vec2::new(1000.0, 1000.0));

    for sprite_bundle in
        chess_floor::bundles::make_sprite_bundles(chess_floor::config::ChessFloorConfig {
            area: spawn_zone,
            tile_size: 20.0,
            ..Default::default()
        })
        .into_iter()
    {
        commands.spawn(sprite_bundle);
    }

    commands.spawn(creature::CreatureConfig::player());

    let npc_chunk_size = Vec2::new(200.0, 200.0);

    for area in RectExt::from(spawn_zone).chunk_to_grid(npc_chunk_size) {
        commands.spawn(creature::CreatureConfig::npc(area.center()));
    }

    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(ScreenState::Gameplay),
    ));
}
