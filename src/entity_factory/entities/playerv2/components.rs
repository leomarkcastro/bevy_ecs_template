use bevy::prelude::Component;

// To be used as data for the playerv1 entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component)]
pub struct InputBind {
    pub active: bool,
}

impl Default for InputBind {
    fn default() -> Self {
        Self { active: false }
    }
}
