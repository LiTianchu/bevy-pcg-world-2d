use crate::{
    game::{
        components::{Movable, ObjectOnGrid},
        constants,
        player::components::Player,
    },
    pcg::{terrain, terrain::tile},
};
use bevy::prelude::*;
use std::collections::HashSet;

pub fn spawn_player(mut commands: Commands, terrain: Res<terrain::resources::TerrainWorld>) {
    let player: Player = Player::new("Player");

    let chunk: &terrain::resources::TerrainChunk = terrain.chunk_at(IVec2 { x: 0, y: 0 }).unwrap();

    let tiles: HashSet<UVec2> = chunk.tiles_of_type(terrain::tile::Tile::Floor);

    let default_spawn_place: UVec2 = UVec2 { x: 0, y: 0 };
    let spawn_place: UVec2 = tiles.iter().next().copied().unwrap_or(default_spawn_place);

    println!(
        "Player spawned: {}, Position: {}",
        player.name(),
        spawn_place
    );

    let spawn_translation: Vec3 =
        terrain::utils::cell_to_pos_local(spawn_place.x as usize, spawn_place.y as usize)
            .with_z(1.0);

    let player_initial_speed: f32 = 100.0;

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

pub fn render_ascii(
    terrain: Res<terrain::resources::TerrainWorld>,
    player_query: Query<&Transform, (With<ObjectOnGrid>, With<Player>, With<Movable>)>,
) {
    let default_transform: Transform = Transform::default();
    let player_transform: &Transform = player_query.single().unwrap_or(&default_transform);
    let (chunk_coord, local_tile_coord) =
        terrain::utils::pos_to_cell_world(player_transform.translation, &terrain);

    let world_ivec2: IVec2 = terrain::utils::get_world_ivec2(chunk_coord, local_tile_coord);

    // compute ascii camera rect
    let camera_width: u32 = constants::ASCII_CAMERA_SIZE * constants::ASCII_CAMERA_ASPECT_RATIO.x;
    let camera_height: u32 = constants::ASCII_CAMERA_SIZE * constants::ASCII_CAMERA_ASPECT_RATIO.y;
    let camera_extent: UVec2 = UVec2 {
        x: camera_width / 2,
        y: camera_height / 2,
    };

    let mut output: Vec<String> = Vec::new();
    for y in (world_ivec2.y - camera_extent.y as i32)..=(world_ivec2.y + camera_extent.y as i32) {
        let mut line: String = String::new();
        for x in (world_ivec2.x - camera_extent.x as i32)..=(world_ivec2.x + camera_extent.x as i32)
        {
            let tile_char: char = terrain
                .tile_at_world_ivec2(IVec2 { x, y })
                .map_or(' ', |t| tile::tile_appearance_ascii(t));

            line.push(tile_char);
        }
        output.push(line);
    }

    // reverse the print as first row in the data is the bottom row in the render
    for line in output.iter().rev() {
        println!("{}", line);
    }
}
