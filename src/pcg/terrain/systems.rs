use crate::pcg::terrain::{constants, resources::TerrainChunk, tile::tile_appearance, utils};

use bevy::prelude::*;

pub fn draw_terrain_chunk(mut commands: Commands, terrain_chunk: Res<TerrainChunk>) {
    for (coord, tile) in terrain_chunk.tiles_iter() {
        commands.spawn((
            Sprite {
                color: tile_appearance(tile),
                custom_size: Some(constants::TILE_DIMESNION),
                ..default()
            },
            Transform::from_translation(utils::cell_coord_to_pos(
                coord.x as usize,
                coord.y as usize,
            )),
        ));
    }
}

pub fn print_terrain_chunk(terrain_chunk: Res<TerrainChunk>) {
    println!("{}", terrain_chunk.to_string());
}
