use bevy::prelude::Component;

// To be used as data for the bulletv1 entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component)]
pub struct Bulletv1Component {
    pub data: String,
    pub printed: bool,
}

#[derive(Component)]
pub struct DissapearAtDistance {
    pub max_distance: f32,
    pub current_distance: f32,
}

impl Default for DissapearAtDistance {
    fn default() -> Self {
        Self {
            max_distance: 100.0,
            current_distance: 0.0,
        }
    }
}
