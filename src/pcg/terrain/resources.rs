use crate::pcg::terrain::{tile, tile::Tile};
use bevy::prelude::*;
use std::collections::HashSet;
use std::fmt::{self, Write};

// Terrain chunk for wave function collapse generation, pivoted at lower left corner (0, 0)
#[derive(Resource)]
pub struct TerrainChunk {
    tiles: Vec<Vec<Tile>>,
}

impl TerrainChunk {
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![]],
        }
    }

    pub fn with_tiles(mut self, tiles: Vec<Vec<Tile>>) -> Self {
        self.tiles = tiles;
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
        for row in self.tiles.iter().rev() {
            for &t in row {
                f.write_char(tile::tile_appearance_ascii(t))?;
            }
            writeln!(f)?
        }

        Ok(())
    }
}
