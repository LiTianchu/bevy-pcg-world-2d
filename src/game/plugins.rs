use crate::{game, pcg::terrain};
use bevy::prelude::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(terrain::utils::generate_terrain(
            terrain::constants::DEFAULT_CHUNK_WIDTH,
            terrain::constants::DEFAULT_CHUNK_HEIGHT,
        ))
        .add_systems(
            Startup,
            (
                game::camera::systems::setup_camera,
                terrain::systems::draw_terrain_chunk,
                terrain::systems::print_terrain_chunk,
                game::systems::spawn_player,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                game::control::systems::handle_player_movement,
                game::camera::systems::camera_follow_player,
            )
                .chain(),
        );
    }
}
