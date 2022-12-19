use bevy::prelude::Resource;

// To be used as data for the face_axis entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Resource, Debug)]
pub struct FaceAxisResource {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Default for FaceAxisResource {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
        }
    }
}
