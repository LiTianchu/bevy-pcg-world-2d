use crate::{game, pcg::terrain};
use bevy::prelude::*;

pub struct WorldPlugins;
impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                terrain::systems::generate_terrain,
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
                terrain::systems::try_regenerate_terrain_around_player,
            )
                .chain(),
        );
    }
}
