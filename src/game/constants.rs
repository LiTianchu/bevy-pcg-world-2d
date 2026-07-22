use bevy::prelude::*;
pub const INITIAL_CAMERA_POSITION: Vec3 = Vec3::new(0.0, 0.0, 10.0);
pub const ASCII_CAMERA_ASPECT_RATIO: UVec2 = UVec2 { x: 16, y: 9 };
pub const ASCII_CAMERA_SIZE: u32 = 5;
pub const DEFAULT_PLAYER_NAME: &str = "Player";
pub const DEFAULT_PLAYER_ASCII_APPEARANCE: char = '@';
pub const DEFAULT_PLAYER_ASCII_COLOR: crossterm::style::Color = crossterm::style::Color::Red;
