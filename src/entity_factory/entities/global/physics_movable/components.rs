use bevy::prelude::{Component, Vec2};

// To be used as data for the physics_movable entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component, Debug)]
pub struct PXMovableComponent {
    pub vec_x: f32,
    pub vec_y: f32,
    pub angle: f32,
}

#[derive(Component, Debug)]
pub struct PXSize {
    pub width: f32,
    pub height: f32,
}

impl Default for PXMovableComponent {
    fn default() -> Self {
        Self {
            vec_x: 0.0,
            vec_y: 0.0,
            angle: 0.0,
        }
    }
}

#[derive(Component, Debug, Copy, Clone)]
pub struct PXConstantMovement {
    pub speed: f32,
    pub angle: f32,
}
