//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    components::chess_floor,
    features::creature,
    screens::ScreenState,
    shared::data_structures::{ChunkToGrid, PrimitiveRect},
};

pub(super) fn plugin(_: &mut App) {
    // app.register_type::<LevelAssets>();
    // app.load_resource::<LevelAssets>();
}

// #[derive(Resource, Asset, Clone, Reflect)]
// #[reflect(Resource)]
// pub struct LevelAssets {
//     #[dependency]
//     music: Handle<AudioSource>,
// }

// impl FromWorld for LevelAssets {
//     fn from_world(world: &mut World) -> Self {
//         let assets = world.resource::<AssetServer>();
//         Self {
//             music: assets.load("audio/music/Fluffing A Duck.ogg"),
//         }
//     }
// }

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    // level_assets: Res<LevelAssets>,
    // player_assets: Res<PlayerAssets>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for sprite_bundle in
        chess_floor::bundles::make_sprite_bundles(chess_floor::config::ChessFloorConfig {
            area: Rect::from_center_size(Vec2::ZERO, Vec2::new(2000.0, 2000.0)),
            tile_size: 20.0,
            ..Default::default()
        })
        .into_iter()
    {
        commands.spawn(sprite_bundle);
    }

    commands.spawn(creature::CreatureConfig::player());

    let npcs_area = Rect::from_corners(Vec2::new(-200.0, -200.0), Vec2::new(200.0, 200.0));
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

// player(400.0, &player_assets, &mut texture_atlas_layouts),
// (
// Name::new("Gameplay Music"),
// music(level_assets.music.clone())
// )
