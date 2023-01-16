use bevy::prelude::Component;

// To be used as data for the animated entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component, Default, Debug, Clone)]
pub struct AnimatedComponent {
    pub index_start: u32,
    pub index_end: u32,
    pub direction: u32,
}
