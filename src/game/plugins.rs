use crate::{game, pcg::terrain};
use bevy::prelude::*;

pub struct WorldPlugins;
impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app.insert_resource(terrain::utils::generate_terrain().with_seed(69))
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
