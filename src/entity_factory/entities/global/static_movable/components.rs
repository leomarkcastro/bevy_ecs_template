use bevy::prelude::Component;

// To be used as data for the global entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component)]
pub struct MovableComponent {
    pub vec_x: f32,
    pub vec_y: f32,
}

impl Default for MovableComponent {
    fn default() -> Self {
        Self {
            vec_x: 0.0,
            vec_y: 0.0,
        }
    }
}
