use crate::{
    game::{components::ObjectOnGrid, player::components::Player},
    pcg::terrain::{
        constants, resources::TerrainSeed, resources::TerrainWorld, tile::tile_color, utils,
    },
};

use bevy::prelude::*;

pub fn generate_terrain(mut command: Commands, seed: Res<TerrainSeed>) {
    command.insert_resource(utils::generate_terrain().with_seed(seed.0));
}

pub fn try_regenerate_terrain_around_player(
    mut terrain: ResMut<TerrainWorld>,
    player_query: Query<&Transform, (With<Player>, With<ObjectOnGrid>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        let (chunk_coord_below, _local_coord_below) =
            utils::pos_to_cell_world(player_transform.translation, &terrain);

        print!("Chunk coord below: {}", chunk_coord_below);
        let should_generate: bool = !terrain.is_chunk_coord_at_cluster_center(chunk_coord_below);
        print!("Should Regenerate: {}", should_generate);
        if should_generate {
            terrain.generate_chunk_cluster_at(
                chunk_coord_below,
                constants::DEFAULT_CHUNK_CLUSTER_EXTENT,
            );
        }
    }
}

pub fn draw_terrain(mut commands: Commands, terrain: Res<TerrainWorld>) {
    let terrain: &TerrainWorld = &terrain;
    for (chunk_coord, chunk) in terrain.chunks_iter() {
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
}
