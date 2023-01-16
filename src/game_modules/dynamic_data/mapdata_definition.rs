use bevy::{
    prelude::{Component, Handle, Resource, Vec2},
    reflect::TypeUuid,
    utils::HashMap,
};

// To be used as data for the dynamic_data entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct PointData {
    pub point_type: String,
    pub center: Vec2,
    pub id: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct LandFeatureData {
    pub id: String,
    pub start: Vec2,
    pub points: Vec<Vec2>,
    pub points_less: Vec<Vec2>,
    pub center: Vec2,
    pub radius: f32,
    pub points_data: Option<Vec<PointData>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BuildingData {
    pub id: String,
    pub center: Vec2,
    pub width: f32,
    pub height: f32,
    pub radius: f32,
    pub bldg_type: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, TypeUuid, Clone)]
#[uuid = "fe5465fe-fe5f-fe5e-fe56-fe62f1e32ee3"]
pub struct MapDataDefinition {
    pub mountain_list_vectorpoints: Vec<LandFeatureData>,
    pub grayroad_list_vectorpoints: Vec<LandFeatureData>,
    pub forest_list_vectorpoints: Vec<LandFeatureData>,
    pub grassfield_list_vectorpoints: Vec<LandFeatureData>,
    pub land_vectorpoints_outline: LandFeatureData,
    pub sand_vectorpoints_outline: LandFeatureData,
    pub buildings: Vec<BuildingData>,
}
