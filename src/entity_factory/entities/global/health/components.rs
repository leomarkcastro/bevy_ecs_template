use bevy::prelude::Component;

// To be used as data for the health entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component)]
pub struct HealthComponent {
    pub health: f32,
}

impl Default for HealthComponent {
    fn default() -> Self {
        Self { health: 100.0 }
    }
}
