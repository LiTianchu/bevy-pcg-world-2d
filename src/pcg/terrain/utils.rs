use crate::pcg::terrain::{
    constants::{self, DEFAULT_CHUNK_CLUSTER_EXTENT},
    resources::TerrainWorld,
};
use bevy::prelude::*;

pub fn generate_terrain() -> TerrainWorld {
    let mut terrain_world: TerrainWorld = TerrainWorld::new();
    terrain_world
        .generate_chunk_cluster_at(constants::INITIAL_CHUNK_COORD, DEFAULT_CHUNK_CLUSTER_EXTENT);
    terrain_world
}
// take in local cell coord within chunk, translate to local pos within chunk
pub fn cell_to_pos_local(x: usize, y: usize) -> Vec3 {
    return Vec3 {
        x: x as f32 * constants::TILE_SIZE,
        y: y as f32 * constants::TILE_SIZE,
        z: 0.0,
    };
}

// take in local pos within chunk, translate to local cell coord within chunk
pub fn pos_to_cell_local(pos: Vec3) -> UVec2 {
    let x: u32 = (pos.x / constants::TILE_SIZE).round() as u32;
    let y: u32 = (pos.y / constants::TILE_SIZE).round() as u32;
    return UVec2 { x, y };
}

// round pos to cell
pub fn cell_round(pos: Vec3) -> Vec3 {
    let x: f32 = (pos.x / constants::TILE_SIZE).round() * constants::TILE_SIZE;
    let y: f32 = (pos.y / constants::TILE_SIZE).round() * constants::TILE_SIZE;
    return Vec3 { x, y, z: pos.z };
}

pub fn cell_to_pos_world(x: usize, y: usize, chunk_coord: IVec2, terrain: &TerrainWorld) -> Vec3 {
    let chunk_dimension: UVec2 = terrain.chunk_dimension();
    let chunk_origin: Vec3 = Vec3 {
        x: chunk_coord.x as f32 * chunk_dimension.x as f32 * constants::TILE_SIZE,
        y: chunk_coord.y as f32 * chunk_dimension.y as f32 * constants::TILE_SIZE,
        z: 0.0,
    };

    chunk_origin + cell_to_pos_local(x, y)
}
