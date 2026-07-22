use bevy::prelude::*;

pub const INITIAL_CHUNK_COORD: IVec2 = IVec2::ZERO;
pub const DEFAULT_CHUNK_CLUSTER_EXTENT: u32 = 1;
pub const DEFAULT_CHUNK_WIDTH: u32 = 64;
pub const DEFAULT_CHUNK_HEIGHT: u32 = 64;
pub const TILE_SIZE: f32 = 16.0;
pub const TILE_DIMESNION: Vec2 = Vec2::splat(TILE_SIZE);
pub const DEFAULT_TERRAIN_WORLD_SEED: u32 = 69;
pub const PERLIN_NOISE_SCALE: f64 = 0.005;
#[allow(dead_code)]
pub const DEFAULT_CHUNK_CENTER: Vec2 = Vec2 {
    x: (DEFAULT_CHUNK_WIDTH as f32 * TILE_SIZE) / 2.0,
    y: (DEFAULT_CHUNK_HEIGHT as f32 * TILE_SIZE) / 2.0,
};
pub const DEFAULT_TILE_PATH: &str = "tiles/white.png";
