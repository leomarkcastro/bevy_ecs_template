use bevy::prelude::Component;

// To be used as data for the physics_movable entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component)]
pub struct PXMovableComponent {
    pub vec_x: f32,
    pub vec_y: f32,
    pub angle: f32,
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
