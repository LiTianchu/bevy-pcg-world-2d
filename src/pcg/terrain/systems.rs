use crate::{
    game::{components::ObjectOnGrid, player::components::Player},
    pcg::terrain::{
        constants,
        resources::{RenderedChunk, TerrainChunk, TerrainSeed, TerrainWorld},
        tile::tile_color,
        utils,
    },
};

use bevy::prelude::*;
use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};
use std::collections::HashSet;

pub fn generate_terrain(mut command: Commands, seed: Res<TerrainSeed>) {
    command.insert_resource(utils::generate_terrain(seed.0));
}

pub fn try_regenerate_terrain_around_player(
    mut terrain: ResMut<TerrainWorld>,
    player_query: Query<&Transform, (With<Player>, With<ObjectOnGrid>, Changed<Transform>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        let (chunk_coord_below, _local_coord_below) =
            utils::pos_to_cell_world(player_transform.translation, &terrain);

        let should_generate: bool = !terrain.is_chunk_coord_at_cluster_center(chunk_coord_below);

        if should_generate {
            terrain.generate_chunk_cluster_at(
                chunk_coord_below,
                constants::DEFAULT_CHUNK_CLUSTER_EXTENT,
            );
            terrain.free_chunks_outside_cluster_at(
                chunk_coord_below,
                constants::DEFAULT_CHUNK_CLUSTER_EXTENT,
            );
        }
    }
}

pub fn draw_terrain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    terrain: Res<TerrainWorld>,
    rendered_chunks: Query<(Entity, &RenderedChunk)>,
    mut cached_tileset: Local<Option<Handle<Image>>>,
) {
    let tileset = cached_tileset
        .get_or_insert_with(|| {
            asset_server
                .load_builder()
                .with_settings(|settings: &mut ImageLoaderSettings| {
                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 2 });
                })
                .load(constants::DEFAULT_TILE_PATH)
        })
        .clone();

    let mut rendered_chunk_coords = HashSet::new();

    for (entity, rendered_chunk) in &rendered_chunks {
        if terrain.chunk_at(rendered_chunk.0).is_none()
            || !rendered_chunk_coords.insert(rendered_chunk.0)
        {
            commands.entity(entity).despawn();
        }
    }

    for (chunk_coord, chunk) in terrain.chunks_iter() {
        if !rendered_chunk_coords.contains(chunk_coord) {
            draw_chunk(
                &mut commands,
                chunk,
                *chunk_coord,
                &terrain,
                tileset.clone(),
            );
        }
    }
}

fn draw_chunk(
    commands: &mut Commands,
    chunk: &TerrainChunk,
    chunk_coord: IVec2,
    terrain: &TerrainWorld,
    tileset: Handle<Image>,
) {
    // build tile data for the chunk
    let tile_data = chunk
        .tiles_iter()
        .map(|(_, tile)| {
            Some(TileData {
                color: tile_color(tile),           // set tile color based on tile type
                ..TileData::from_tileset_index(0)  // select timemap layer 0
            })
        })
        .collect();

    let chunk_size = chunk.dimension();

    // TilemapChunk is centered, while the current world coordinates place tile (0, 0) at the chunk origin.
    let origin = utils::cell_to_pos_world(0, 0, chunk_coord, terrain);
    let center_offset = (chunk_size.as_vec2() - Vec2::ONE) * constants::TILE_SIZE * 0.5;

    commands.spawn((
        RenderedChunk::new(chunk_coord),
        TilemapChunk {
            chunk_size,
            tile_display_size: UVec2::splat(constants::TILE_SIZE as u32),
            tileset,
            ..default()
        },
        TilemapChunkTileData(tile_data),
        Transform::from_translation(origin + center_offset.extend(0.0)),
    ));
}
