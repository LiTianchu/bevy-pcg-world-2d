use crate::{game, pcg::terrain};
use bevy::prelude::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(terrain::utils::generate_terrain())
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
