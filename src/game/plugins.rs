use crate::{game, pcg::terrain};
use bevy::prelude::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let seed: u32 = 1234;
        app.insert_resource(terrain::utils::generate_terrain(
            seed,
            terrain::constants::DEFAULT_GRID_WIDTH,
            terrain::constants::DEFAULT_GRID_HEIGHT,
        ))
        .add_systems(
            Startup,
            (
                game::camera::systems::setup_camera,
                terrain::systems::draw_terrain,
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
