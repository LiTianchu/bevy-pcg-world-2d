use crate::{
    game::{
        components::{Movable, ObjectOnGrid},
        player::components::Player,
    },
    pcg::terrain,
};
use bevy::prelude::*;
use std::collections::HashSet;

pub fn spawn_player(mut commands: Commands, chunk: Res<terrain::resources::TerrainChunk>) {
    let player: Player = Player::new("Player");
    let tiles: HashSet<UVec2> = chunk.tiles_of_type(terrain::tile::Tile::Floor);

    let default_spawn_place: UVec2 = UVec2 { x: 0, y: 0 };
    let spawn_place: UVec2 = tiles.iter().next().copied().unwrap_or(default_spawn_place);

    println!(
        "Player spawned: {}, Position: {}",
        player.name(),
        spawn_place
    );

    let spawn_translation: Vec3 =
        terrain::utils::cell_coord_to_pos(spawn_place.x as usize, spawn_place.y as usize)
            .with_z(1.0);

    let player_initial_speed: f32 = 7.0;

    commands.spawn((
        player,
        Transform::from_translation(spawn_translation),
        Sprite {
            color: Color::srgba(0.8, 0.2, 0.1, 1.0),
            custom_size: Some(terrain::constants::TILE_DIMESNION),
            ..default()
        },
        Movable::new().with_speed(player_initial_speed),
        ObjectOnGrid::new().with_internal_translation(spawn_translation),
    ));
}
