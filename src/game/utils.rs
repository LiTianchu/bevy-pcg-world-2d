use crate::{
    game::components::{Movable, ObjectOnGrid},
    pcg::terrain,
};
use bevy::prelude::*;

pub fn try_move(
    transform: &mut Transform,
    movable: &mut Movable,
    object_on_grid: &mut ObjectOnGrid,
    direction: Vec2,
    time: &Res<Time>,
    terrain: &Res<terrain::resources::TerrainWorld>,
) -> Option<Vec3> {
    // let terrain_chunk: &terrain::resources::TerrainChunk =
    //     terrain.chunk_of_pos(transform.translation).1.unwrap();
    let move_interval: f32 = movable.move_interval();

    // if last step time is reset or enough time has passed since the last step
    let move_not_in_cooldown = movable.last_step_time.map_or(true, |last_time| {
        time.elapsed_secs_f64() - last_time >= move_interval as f64
    });

    // precompute the next grid pos
    let next_grid_pos: Vec3 = terrain::utils::cell_round(
        Vec3 {
            x: direction.x * terrain::constants::TILE_SIZE,
            y: direction.y * terrain::constants::TILE_SIZE,
            z: 0.0,
        } + object_on_grid.internal_translation,
    );

    let (chunk_coord, next_cell_coord) = terrain::utils::pos_to_cell_world(next_grid_pos, &terrain);

    let is_tile_walkable = terrain
        .is_tile_walkable(chunk_coord, next_cell_coord)
        .unwrap_or(false);

    // execute the move only if the move is not in cooldown and the next tile is walkable
    if move_not_in_cooldown && is_tile_walkable {
        // update the step time to restart cooldown
        if next_grid_pos != transform.translation {
            movable.last_step_time = Some(time.elapsed_secs_f64());
        }

        object_on_grid.internal_translation = next_grid_pos;
        transform.translation = next_grid_pos;
        Some(next_grid_pos)
    } else {
        None
    }
}
