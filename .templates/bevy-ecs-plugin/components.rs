use bevy::prelude::Component;

// To be used as data for the __templateName__ entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component)]
pub struct __templateNameToPascalCase__Component {
    pub data: String,
    pub printed: bool,
}
