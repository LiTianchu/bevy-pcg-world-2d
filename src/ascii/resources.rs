use bevy::prelude::*;
use crossterm::event::KeyCode as CrosstermKeyCode;
#[derive(Resource, Default)]
pub struct TerminalInput {
    pub pressed_key: Option<CrosstermKeyCode>,
}
