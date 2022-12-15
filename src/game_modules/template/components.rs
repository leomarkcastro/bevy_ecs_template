use bevy::prelude::Component;

// To be used as data for the template entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component)]
pub struct TemplateComponent {
    pub data: String,
    pub printed: bool,
}
