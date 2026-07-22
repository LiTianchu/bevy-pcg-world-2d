use crate::pcg::terrain::{
    constants, resources::TerrainSeed, resources::TerrainWorld, tile::tile_color, utils,
};

use bevy::prelude::*;

pub fn generate_terrain(mut command: Commands, seed: Res<TerrainSeed>) {
    command.insert_resource(utils::generate_terrain().with_seed(seed.0));
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
