use bevy::{prelude::Component, utils::Uuid};

// To be used as data for the despawn entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component, Debug, Clone)]
pub struct DespawnWithTimerComponent {
    pub id: String,
    pub despawn_on: f32,
}

impl Default for DespawnWithTimerComponent {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            despawn_on: 0.0,
        }
    }
}
