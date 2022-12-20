// To describe how the Health component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::HealthComponent;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(health_init_system)
            .add_system(health_system);
    }
}

fn health_init_system() {}

fn health_system() {}
