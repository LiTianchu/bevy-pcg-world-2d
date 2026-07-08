use crate::pcg::terrain::{constants, resources::Terrain, tile, tile::Tile};
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

pub fn generate_terrain(grid_width: usize, grid_height: usize) -> Terrain {
    let mut rng = rand::rng();
    let seed: u32 = rng.random();
    return generate_terrain_seeded(seed, grid_width, grid_height);
}
pub fn generate_terrain_seeded(seed: u32, grid_width: usize, grid_height: usize) -> Terrain {
    let perlin = Perlin::new(seed);
    let mut terrain_tiles = vec![vec![Tile::Void; grid_width]; grid_height];

    let scale = 0.1;
    for y in 0..grid_height {
        for x in 0..grid_width {
            let value = perlin.get([x as f64 * scale, y as f64 * scale]);
            terrain_tiles[y][x] = tile::get_tile_by_f64(value);
        }
    }

    return Terrain::new().with_tiles(terrain_tiles);
}

pub fn cell_coord_to_pos(x: usize, y: usize) -> Vec3 {
    return Vec3 {
        x: x as f32 * constants::TILE_SIZE,
        y: y as f32 * constants::TILE_SIZE,
        z: 0.0,
    };
}

pub fn pos_to_cell_coord(pos: Vec3) -> UVec2 {
    let x: u32 = (pos.x / constants::TILE_SIZE).round() as u32;
    let y: u32 = (pos.y / constants::TILE_SIZE).round() as u32;
    return UVec2 { x, y };
}

pub fn round_pos_to_cell(pos: Vec3) -> Vec3 {
    let x: f32 = (pos.x / constants::TILE_SIZE).round() * constants::TILE_SIZE;
    let y: f32 = (pos.y / constants::TILE_SIZE).round() * constants::TILE_SIZE;
    return Vec3 { x, y, z: pos.z };
}
