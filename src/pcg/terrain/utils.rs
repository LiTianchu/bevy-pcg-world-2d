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

pub fn pos_to_cell_world(pos: Vec3, terrain: &TerrainWorld) -> (IVec2, UVec2) {
    let (chunk_coord, chunk_option) = terrain.chunk_of_pos(pos);
    chunk_option.map_or((IVec2::ZERO, UVec2::ZERO), |_chunk| {
        (chunk_coord, pos_to_cell_local(terrain.chunk_mod_pos(pos)))
    })
}

pub fn get_world_ivec2(chunk_coord: IVec2, cell_coord: UVec2) -> IVec2 {
    let x: i32 = chunk_coord.x * constants::DEFAULT_CHUNK_WIDTH as i32 + cell_coord.x as i32;
    let y: i32 = chunk_coord.y * constants::DEFAULT_CHUNK_HEIGHT as i32 + cell_coord.y as i32;
    return IVec2 { x, y };
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
