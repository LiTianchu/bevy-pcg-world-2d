use crate::pcg::terrain::{constants, resources::Terrain, tile::tile_appearance, utils};

use bevy::prelude::*;

pub fn draw_terrain(mut commands: Commands, terrain: Res<Terrain>) {
    for (coord, tile) in terrain.tiles_iter() {
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
