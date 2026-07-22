use crate::{
    ascii::{self, resources::TerminalInput},
    game,
    pcg::terrain,
};
use bevy::prelude::*;

pub struct AsciiWorldPlugins;
impl Plugin for AsciiWorldPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerminalInput>()
            .add_systems(
                Startup,
                (
                    terrain::systems::generate_terrain,
                    ascii::systems::setup_terminal,
                    game::systems::spawn_player,
                )
                    .chain(),
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
