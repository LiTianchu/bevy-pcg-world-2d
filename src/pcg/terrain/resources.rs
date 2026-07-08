use crate::pcg::terrain::utils;
use crate::pcg::terrain::{constants, tile, tile::Tile};
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rustc_hash::FxHasher;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Write};
use std::hash::{Hash, Hasher};

#[derive(Resource)]
pub struct TerrainWorld {
    chunks: HashMap<IVec2, TerrainChunk>,
    chunk_dimension: UVec2,
    seed: u32,
    perlin: Perlin,
}

impl TerrainWorld {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            chunk_dimension: UVec2 {
                x: constants::DEFAULT_CHUNK_WIDTH,
                y: constants::DEFAULT_CHUNK_HEIGHT,
            },
            seed: constants::DEFAULT_TERRAIN_WORLD_SEED,
            perlin: Perlin::new(constants::DEFAULT_TERRAIN_WORLD_SEED),
        }
    }

    pub fn with_seed(mut self, seed: u32) -> Self {
        self.seed = seed;
        self.perlin = Perlin::new(seed);
        self
    }

    pub fn chunk_at(&self, coord: IVec2) -> Option<&TerrainChunk> {
        self.chunks.get(&coord)
    }

    pub fn chunk_of_pos(&self, pos: Vec3) -> Option<&TerrainChunk> {
        let chunk_x = (pos.x / constants::TILE_SIZE / self.chunk_dimension.x as f32).floor() as i32;
        let chunk_y = (pos.y / constants::TILE_SIZE / self.chunk_dimension.y as f32).floor() as i32;
        self.chunk_at(IVec2 {
            x: chunk_x,
            y: chunk_y,
        })
    }

    pub fn chunk_dimension(&self) -> UVec2 {
        self.chunk_dimension.clone()
    }

    pub fn chunks_iter(&self) -> impl Iterator<Item = (&IVec2, &TerrainChunk)> + '_ {
        self.chunks.iter()
    }

    pub fn generate_chunk_cluster_at(&mut self, center_chunk: IVec2, extent: u32) {
        for x in (center_chunk.x - extent as i32)..=(center_chunk.x + extent as i32) {
            for y in (center_chunk.y - extent as i32)..=(center_chunk.y + extent as i32) {
                self.generate_chunk_if_not_exist(IVec2 { x, y });
            }
        }
    }

    pub fn generate_chunk_if_not_exist(&mut self, coord: IVec2) {
        if self.chunks.contains_key(&coord) {
            return;
        }

        // stable chunk seed based on coordinate
        // ensures same chunk coordinate always generates the same terrain in WFC generation

        let new_chunk: TerrainChunk = self.get_new_chunk(
            coord,
            self.chunk_dimension.x as usize,
            self.chunk_dimension.y as usize,
        );

        self.chunks.entry(coord).or_insert(new_chunk);
    }

    fn compute_chunk_seed(&self, coord: IVec2) -> u32 {
        let mut hasher = FxHasher::default();
        (self.seed, coord.x, coord.y).hash(&mut hasher);
        let seed_u64: u64 = hasher.finish();

        // shorten chunk_seed to u32 using XOR fold for Perlin noise
        (seed_u64 as u32) ^ ((seed_u64 >> 32) as u32)
    }

    fn get_new_chunk(
        &mut self,
        chunk_coord: IVec2,
        grid_width: usize,
        grid_height: usize,
    ) -> TerrainChunk {
        let chunk_seed: u32 = self.compute_chunk_seed(chunk_coord);
        let mut terrain_tiles = vec![vec![Tile::Void; grid_width]; grid_height];

        for y in 0..grid_height {
            for x in 0..grid_width {
                let xy_global = utils::cell_to_pos_world(x, y, chunk_coord, self);
                let value = self.perlin.get([
                    xy_global.x as f64 * constants::PERLIN_NOISE_SCALE,
                    xy_global.y as f64 * constants::PERLIN_NOISE_SCALE,
                ]);
                terrain_tiles[y][x] = tile::get_tile_by_f64(value);
            }
        }

        return TerrainChunk::new()
            .with_tiles(terrain_tiles)
            .with_seed(chunk_seed);
    }
}

// Terrain chunk for wave function collapse generation, pivoted at lower left corner (0, 0)
#[derive(Resource)]
pub struct TerrainChunk {
    tiles: Vec<Vec<Tile>>,
    seed: u32,
}

impl TerrainChunk {
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![]],
            seed: 67,
        }
    }

    pub fn with_tiles(mut self, tiles: Vec<Vec<Tile>>) -> Self {
        self.tiles = tiles;
        self
    }

    pub fn with_seed(mut self, seed: u32) -> Self {
        self.seed = seed;
        self
    }

    #[allow(dead_code)]
    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) -> Result {
        let curr_tile: Tile = self.tile(x, y)?;
        if curr_tile != tile {
            self.tiles[y][x] = tile;
        }
        Ok(())
    }

    pub fn dimension(&self) -> UVec2 {
        if self.tiles.len() == 0 {
            UVec2 { x: 0, y: 0 }
        } else {
            UVec2 {
                x: self.tiles[0].len() as u32,
                y: self.tiles.len() as u32,
            }
        }
    }

    pub fn tiles_iter(&self) -> impl Iterator<Item = (UVec2, Tile)> + '_ {
        self.tiles.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &tile)| (UVec2::new(x as u32, y as u32), tile))
        })
    }

    pub fn tile(&self, x: usize, y: usize) -> Result<Tile> {
        if self.tiles.len() == 0 {
            return Err("Grid is empty".into());
        }
        if y < self.tiles.len() && x < self.tiles[0].len() {
            Ok(self.tiles[y][x])
        } else {
            Err("Tile coordinate out of boundary".into())
        }
    }

    pub fn tiles_of_type(&self, tile_type: Tile) -> HashSet<UVec2> {
        let dim: UVec2 = self.dimension();
        let mut found_set: HashSet<UVec2> = HashSet::new();
        for y in 0..dim.y {
            for x in 0..dim.x {
                if self.tiles[y as usize][x as usize] == tile_type {
                    found_set.insert(UVec2 { x: x, y: y });
                }
            }
        }
        return found_set;
    }

    pub fn is_tile_walkable(&self, x: usize, y: usize) -> Result<bool> {
        let tile: Tile = self.tile(x, y)?;
        match tile {
            Tile::Floor => Ok(true),
            Tile::Grass => Ok(true),
            _ => Ok(false),
        }
    }
}

impl fmt::Display for TerrainChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // need to reverse the rows as 2d map array's first row should be printed at the bottom
        for row in self.tiles.iter().rev() {
            for &t in row {
                f.write_char(tile::tile_appearance_ascii(t))?;
            }
            writeln!(f)?
        }

        Ok(())
    }
}
