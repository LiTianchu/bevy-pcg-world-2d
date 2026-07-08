use crate::{
    game::{
        components::{Movable, ObjectOnGrid},
        player::components::Player,
    },
    pcg::terrain,
};
use bevy::prelude::*;

pub fn handle_player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    terrain: Res<terrain::resources::Terrain>,
    query: Query<(&mut Transform, &Player, &mut Movable, &mut ObjectOnGrid)>,
) {
    let mut direction: Vec2 = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y = 1.0;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y = -1.0;
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x = -1.0;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x = 1.0;
    }

    direction = direction.normalize_or_zero();

    if direction == Vec2::ZERO {
        return;
    }

    for (mut transform, _player, mut movable, mut object_on_grid) in query {
        if keyboard_input.just_released(KeyCode::KeyW)
            || keyboard_input.just_released(KeyCode::KeyS)
            || keyboard_input.just_released(KeyCode::KeyA)
            || keyboard_input.just_released(KeyCode::KeyD)
        {
            movable.last_step_time = None;
        }

        let move_interval: f32 = movable.move_interval();

        let move_not_in_cooldown = movable.last_step_time.map_or(true, |last_time| {
            time.elapsed_secs_f64() - last_time >= move_interval as f64
        });

        // precompute the next grid pos
        let next_grid_pos: Vec3 = terrain::utils::round_pos_to_cell(
            Vec3 {
                x: direction.x * terrain::constants::TILE_SIZE,
                y: direction.y * terrain::constants::TILE_SIZE,
                z: 0.0,
            } + object_on_grid.internal_translation,
        );

        let next_cell_coord: UVec2 = terrain::utils::pos_to_cell_coord(next_grid_pos);

        let is_tile_walkable = terrain
            .is_tile_walkable(next_cell_coord.x as usize, next_cell_coord.y as usize)
            .unwrap_or(false);

        // if last step time is reset or enough time has passed since the last step, move the player
        // by one tile in the dir of movement
        if move_not_in_cooldown && is_tile_walkable {
            if next_grid_pos != transform.translation {
                movable.last_step_time = Some(time.elapsed_secs_f64());
            }

            object_on_grid.internal_translation = next_grid_pos;
            transform.translation = next_grid_pos;
        }
    }
}
