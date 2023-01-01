use bevy::prelude::Component;

// To be used as data for the despawn entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component, Debug, Clone)]
pub struct DespawnComponent {
    pub id: String,
    pub camera_circle: f32,
    pub bldg_circle: f32,
}
