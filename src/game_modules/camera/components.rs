use bevy::{
    prelude::{Component, Entity, Resource, Transform, Vec3},
    utils::Uuid,
};
use bevy_inspector_egui::Inspectable;

// To be used as data for the camera entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

pub enum CameraMode {
    Stay,
    AtPoint {
        target_point: Vec3,
    },
    AtAsset {
        target_asset_id: Uuid,
    },
    AtAssetFace {
        target_asset_id: Uuid,
        distance: f32,
    },
    AtTwoAssets {
        target_a_asset_id: Uuid,
        target_b_asset_id: Uuid,
    },
}

#[derive(Resource)]
pub struct CameraResource {
    pub mode: CameraMode,
    pub speed: f32,
}

impl Default for CameraResource {
    fn default() -> Self {
        Self {
            mode: CameraMode::Stay,
            speed: 1.0,
        }
    }
}

// Anything that camera can follow should have this component
#[derive(Debug, Component, Clone, Copy)]
pub struct CameraFollowable {
    pub id: Uuid,
}

impl Default for CameraFollowable {
    fn default() -> Self {
        // generate uuid here
        let id = Uuid::new_v4();
        Self { id }
    }
}
