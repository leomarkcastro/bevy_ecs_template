use bevy::prelude::Component;

// To be used as data for the zombiesv1 entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component)]
pub struct Zombiesv1Component {
    pub data: String,
    pub printed: bool,
}
