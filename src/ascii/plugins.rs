use crate::{
    ascii::{self, resources::TerminalInput},
    game,
    pcg::terrain,
};
use bevy::prelude::*;

pub struct AsciiWorldPlugin;
impl Plugin for AsciiWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerminalInput>()
            .insert_resource(terrain::utils::generate_terrain().with_seed(69))
            .add_systems(
                Startup,
                (ascii::systems::setup_terminal, game::systems::spawn_player).chain(),
            )
            .add_systems(
                Update,
                (
                    ascii::systems::read_terminal_input,
                    ascii::systems::handle_terminal_player_movement
                        .after(ascii::systems::read_terminal_input),
                    ascii::systems::render_ascii,
                    ascii::systems::handle_terminal_quit_game,
                )
                    .chain(),
            );
    }
}
