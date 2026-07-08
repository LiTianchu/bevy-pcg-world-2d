use bevy::prelude::*;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Water,
    Mountain,
    Grass,
    Floor,
    Void,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wall => write!(f, "Wall"),
            Tile::Water => write!(f, "Water"),
            Tile::Mountain => write!(f, "Mountain"),
            Tile::Grass => write!(f, "Grass"),
            Tile::Floor => write!(f, "Floor"),
            Tile::Void => write!(f, "V"),
        }
    }
}

pub fn get_tile_by_f64(val: f64) -> Tile {
    if val < -0.5 {
        Tile::Void
    } else if val >= -0.5 && val < -0.7 {
        Tile::Mountain
    } else if val >= -0.7 && val < -0.9 {
        Tile::Water
    } else if val >= -0.9 && val < 0.0 {
        Tile::Wall
    } else if val >= 0.0 && val < 0.25 {
        Tile::Grass
    } else {
        Tile::Floor
    }
}
pub fn tile_appearance_ascii(tile: Tile) -> char {
    let tile_ascii: char = match tile {
        Tile::Wall => '#',
        Tile::Water => '~',
        Tile::Mountain => '^',
        Tile::Grass => '"',
        Tile::Floor => '.',
        Tile::Void => ' ',
    };
    tile_ascii
}

pub fn tile_appearance(tile: Tile) -> Color {
    let color: Color = match tile {
        Tile::Wall => Color::srgba_u8(86, 86, 71, 255),
        Tile::Water => Color::srgba_u8(37, 178, 169, 255),
        Tile::Floor => Color::srgba_u8(147, 153, 155, 255),
        Tile::Mountain => Color::srgba_u8(14, 35, 32, 255),
        Tile::Grass => Color::srgba_u8(110, 163, 111, 255),
        Tile::Void => Color::srgba_u8(0, 0, 0, 255),
    };
    color
}
