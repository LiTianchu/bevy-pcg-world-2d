use crate::{
    game::{
        components::{Movable, ObjectOnGrid},
        player::components::Player,
        utils::try_move,
    },
    pcg::terrain,
};
use bevy::prelude::*;

pub fn handle_player_movement(
    time: Res<Time>,
    keyboard_input: Option<Res<ButtonInput<KeyCode>>>,
    terrain: Res<terrain::resources::TerrainWorld>,
    query: Query<(&mut Transform, &mut Movable, &mut ObjectOnGrid), With<Player>>,
) {
    let Some(keyboard_input) = keyboard_input else {
        return;
    };

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

    for (mut transform, mut movable, mut object_on_grid) in query {
        if keyboard_input.just_released(KeyCode::KeyW)
            || keyboard_input.just_released(KeyCode::KeyS)
            || keyboard_input.just_released(KeyCode::KeyA)
            || keyboard_input.just_released(KeyCode::KeyD)
        {
            movable.last_step_time = None;
        }

        try_move(
            &mut transform,
            &mut movable,
            &mut object_on_grid,
            direction,
            &time,
            &terrain,
        );
    }
}
