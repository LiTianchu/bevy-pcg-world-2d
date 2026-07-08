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

    for (mut transform, _player, mut movable, mut object_on_grid) in query {
        if keyboard_input.just_released(KeyCode::KeyW)
            || keyboard_input.just_released(KeyCode::KeyS)
            || keyboard_input.just_released(KeyCode::KeyA)
            || keyboard_input.just_released(KeyCode::KeyD)
        {
            movable.last_step_time = None;
        }

        let move_interval = 1.0 / movable.speed as f64;

        // if last step time is reset or enough time has passed since the last step, move the player
        // by one tile in the dir of movement
        if movable.last_step_time.map_or(true, |last_time| {
            time.elapsed_secs_f64() - last_time >= move_interval
        }) {
            object_on_grid.internal_translation.x += direction.x * terrain::constants::TILE_SIZE;
            object_on_grid.internal_translation.y += direction.y * terrain::constants::TILE_SIZE;
        }

        let next_grid_pos = terrain::utils::pos_to_grid(object_on_grid.internal_translation);

        if next_grid_pos != transform.translation {
            movable.last_step_time = Some(time.elapsed_secs_f64());
        }

        transform.translation = next_grid_pos;
    }
}
