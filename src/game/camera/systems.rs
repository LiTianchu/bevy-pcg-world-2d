use crate::game::{
    components::{Movable, ObjectOnGrid},
    constants,
    player::components::Player,
};
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection::default_2d()),
        Transform::from_translation(constants::INITIAL_CAMERA_POSITION),
    ));
}

pub fn camera_follow_player(
    player_query: Query<&Transform, (With<ObjectOnGrid>, With<Player>, With<Movable>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
