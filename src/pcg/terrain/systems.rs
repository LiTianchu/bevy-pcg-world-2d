use crate::{
    game::{components::ObjectOnGrid, player::components::Player},
    helper::math::IRectExcept,
    pcg::terrain::{
        constants,
        resources::TerrainSeed,
        resources::{TerrainChunk, TerrainWorld},
        tile::tile_color,
        utils,
    },
};
use bevy::prelude::*;

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
            let old_rect: IRect = terrain.compute_chunk_coord_rect();
            terrain.generate_chunk_cluster_at(
                chunk_coord_below,
                constants::DEFAULT_CHUNK_CLUSTER_EXTENT,
            );
            let new_rect: IRect = terrain.compute_chunk_coord_rect();
            let excepted_rects: Vec<IRect> = old_rect.except(&new_rect);
            for excepted_rect in excepted_rects {
                let Some(_removed_chunk_coords) = terrain.free_chunks_at_rect(excepted_rect) else {
                    continue;
                };
            }
        }
    }
}

pub fn draw_terrain(mut commands: Commands, terrain: Res<TerrainWorld>) {
    let terrain: &TerrainWorld = &terrain;
    for (chunk_coord, chunk) in terrain.chunks_iter() {
        draw_chunk(&mut commands, chunk, chunk_coord, terrain);
    }
}

fn draw_chunk(
    commands: &mut Commands,
    chunk: &TerrainChunk,
    chunk_coord: &IVec2,
    terrain: &TerrainWorld,
) {
    for (tile_coord, tile) in chunk.tiles_iter() {
        commands.spawn((
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
}
