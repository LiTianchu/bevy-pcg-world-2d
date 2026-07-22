use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct GameConfig {
    pub collision_enabled: bool,
}

impl GameConfig {
    pub fn new() -> Self {
        GameConfig {
            collision_enabled: true,
        }
    }

    pub fn with_collision_enabled(mut self, enabled: bool) -> Self {
        self.collision_enabled = enabled;
        self
    }
}
