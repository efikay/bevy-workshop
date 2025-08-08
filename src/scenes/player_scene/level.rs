use bevy::prelude::*;

use crate::{
    // components::chess_floor,
    features::creature,
    screens::ScreenState,
    shared::data_structures::{ChunkToGrid, PrimitiveRect},
};

pub(super) fn plugin(_: &mut App) {
    //
}

pub fn spawn_level(mut commands: Commands) {
    // for sprite_bundle in
    //     chess_floor::bundles::make_sprite_bundles(chess_floor::config::ChessFloorConfig {
    //         area: Rect::from_center_size(Vec2::ZERO, Vec2::new(2000.0, 2000.0)),
    //         tile_size: 20.0,
    //         ..Default::default()
    //     })
    //     .into_iter()
    // {
    //     commands.spawn(sprite_bundle);
    // }

    commands.spawn(creature::CreatureConfig::player());

    let npcs_area = Rect::from_corners(Vec2::new(-1000.0, -1000.0), Vec2::new(1000.0, 1000.0));
    let npc_chunk = Vec2::new(200.0, 200.0);

    for spawn_area in
        PrimitiveRect::new(npcs_area.min.into(), npcs_area.max.into()).chunk_to_grid(npc_chunk)
    {
        let center = spawn_area.max() - (npc_chunk / 2.0);

        commands.spawn(creature::CreatureConfig::npc(center));
    }

    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(ScreenState::Gameplay),
    ));
}
