// To describe how the Camera component/entity should behave.
// WILL: contain pure logic that interacts with the component

use std::f32::consts::PI;

use bevy::{math::Vec3Swizzles, prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::RegisterInspectable;

use crate::{
    entity_factory::entities::playerv2::entities::Playerv2Entity,
    game_modules::{pan_camera::components::PanOrbitCamera, timers::components::OneSecondTimer},
};

use super::components::{CameraFollowable, CameraMode, CameraResource};

pub const CLEAR: Color = Color::rgb(1.0, 1.0, 1.0);
pub const HEIGHT: f32 = 600.0;
pub const RESOLUTION: f32 = 4.0 / 3.0;
pub const PROJECTION_SIZE: f32 = 50.0 * 1.25;
// pub const SCALE_MODE: ScalingMode = ScalingMode::None;
pub const SCALE_MODE: ScalingMode = ScalingMode::FixedVertical(PROJECTION_SIZE * RESOLUTION);

pub struct CameraSetupPlugin;

impl Plugin for CameraSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraResource::default())
            .add_startup_system(camera_init_system)
            .add_system(camera_follow_system);
        // .add_system(camera_system);
    }
}

pub fn camera_init_system(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.right = PROJECTION_SIZE * RESOLUTION;
    camera.projection.left = -PROJECTION_SIZE * RESOLUTION;

    camera.projection.top = PROJECTION_SIZE;
    camera.projection.bottom = -PROJECTION_SIZE;

    camera.projection.scaling_mode = SCALE_MODE;

    commands.spawn(camera).insert(PanOrbitCamera {
        ..Default::default()
    });
}

fn camera_follow_system(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    asset_query: Query<(&Transform, &CameraFollowable), Without<Camera2d>>,
    camera_resource: Res<CameraResource>,
) {
    let camera_xy = camera_query.single().translation.xy();

    match (camera_resource.mode) {
        CameraMode::Stay => {}
        CameraMode::AtPoint { target_point } => {
            // move at point from current position
            let target_point_xy = target_point.xy();

            // lertp x and y
            let new_position = camera_xy.lerp(target_point_xy, camera_resource.speed);

            // set new position

            camera_query.single_mut().translation.x = new_position.x;
            camera_query.single_mut().translation.y = new_position.y;
        }
        CameraMode::AtAsset { target_asset_id } => {
            // get first followable asset
            let target_asset_query = asset_query
                .iter()
                .find(|(_, followable)| followable.id == target_asset_id);

            // if no asset found, return
            if target_asset_query.is_none() {
                return;
            }

            let target_asset = target_asset_query.unwrap().0;

            // move at asset from current position
            let target_asset_xy = target_asset.translation.xy();
            let new_position = camera_xy.lerp(target_asset_xy, camera_resource.speed);
            camera_query.single_mut().translation.x = new_position.x;
            camera_query.single_mut().translation.y = new_position.y;
        }
        CameraMode::AtAssetFace {
            target_asset_id,
            distance,
        } => {
            // get first followable asset
            let target_asset_query = asset_query
                .iter()
                .find(|(_, followable)| followable.id == target_asset_id);

            // if no asset found, return
            if target_asset_query.is_none() {
                return;
            }

            let target_asset = target_asset_query.unwrap().0;

            // move at asset from current position
            let target_asset_xy = target_asset.translation.xy();
            let target_asset_angle = target_asset.rotation;

            // quat to vec3
            let target_asset_angle =
                target_asset.rotation.to_axis_angle().1 * target_asset.rotation.to_axis_angle().0.z;

            // get new position with x distrance from target asset and angle
            let target_asset_xy_with_angle = target_asset_xy
                + Vec2::new(
                    distance * target_asset_angle.cos(),
                    distance * target_asset_angle.sin(),
                );

            let new_position = camera_xy.lerp(target_asset_xy_with_angle, camera_resource.speed);
            camera_query.single_mut().translation.x = new_position.x;
            camera_query.single_mut().translation.y = new_position.y;
        }
        CameraMode::AtTwoAssets {
            target_a_asset_id,
            target_b_asset_id,
        } => {}
        _ => {}
    }
}
