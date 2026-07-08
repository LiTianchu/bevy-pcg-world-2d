use bevy::prelude::*;
#[derive(Component)]
pub struct ObjectOnGrid {
    pub internal_translation: Vec3,
}

impl ObjectOnGrid {
    pub fn new() -> Self {
        Self {
            internal_translation: Vec3::ZERO,
        }
    }

    pub fn with_internal_translation(mut self, translation: Vec3) -> Self {
        self.internal_translation = translation;
        self
    }
}

#[derive(Component)]
pub struct Movable {
    pub speed: f32,
    pub last_step_time: Option<f64>,
}

impl Movable {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
            last_step_time: None,
        }
    }

    pub fn with_speed(mut self, new_speed: f32) -> Self {
        self.speed = new_speed;
        self
    }
}
