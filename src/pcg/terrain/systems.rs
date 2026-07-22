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
    terrain: Res<TerrainWorld>,
    rendered_chunks_query: Query<(Entity, &RenderedChunk)>,
) {
    let terrain: &TerrainWorld = &terrain;
    let mut rendered_chunk_coords = HashSet::new();

    for (entity, rendered_chunk) in &rendered_chunks_query {
        // if chunk no longer exists in the terrain
        // or if chunk is already rendered (duplicate)
        if terrain.chunk_at(rendered_chunk.0).is_none()
            || !rendered_chunk_coords.insert(rendered_chunk.0)
        {
            commands.entity(entity).despawn();
        }
    }

    for (chunk_coord, chunk) in terrain.chunks_iter() {
        if !rendered_chunk_coords.contains(chunk_coord) {
            draw_chunk(&mut commands, chunk, chunk_coord, terrain);
        }
    }
}

fn draw_chunk(
    commands: &mut Commands,
    chunk: &TerrainChunk,
    chunk_coord: &IVec2,
    terrain: &TerrainWorld,
) {
    commands
        .spawn((
            RenderedChunk::new(chunk_coord.clone()),
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            for (tile_coord, tile) in chunk.tiles_iter() {
                parent.spawn((
                    Sprite {
                        color: tile_color(tile),
                        custom_size: Some(constants::TILE_DIMESNION),
                        ..default()
                    },
                    Transform::from_translation(utils::cell_to_pos_world(
                        tile_coord.x as usize,
                        tile_coord.y as usize,
                        chunk_coord.clone(),
                        terrain,
                    )),
                ));
            }
        });
}
