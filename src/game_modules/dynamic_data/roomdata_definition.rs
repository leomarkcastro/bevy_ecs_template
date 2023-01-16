use bevy::{
    prelude::{Component, Handle, Resource, Vec2},
    reflect::TypeUuid,
    utils::HashMap,
};

// To be used as data for the dynamic_data entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(serde::Deserialize, Debug)]
pub struct RoomFeatureDescriptionData {
    pub width: f32,
    pub height: f32,
    pub center: Vec2,
    pub element_type: String,
    pub room_code: String,
    pub level: u32,
}

#[derive(serde::Deserialize, Debug)]
pub struct RoomFeatureData {
    pub walls: Vec<RoomFeatureDescriptionData>,
    pub doors: Vec<RoomFeatureDescriptionData>,
    pub roofs: Vec<RoomFeatureDescriptionData>,
    pub crates: Vec<RoomFeatureDescriptionData>,
    pub pickups: Vec<RoomFeatureDescriptionData>,
    pub enemies: Vec<RoomFeatureDescriptionData>,
    pub center: Vec2,
    pub size: Vec2,
}

#[derive(serde::Deserialize, Debug, TypeUuid)]
#[uuid = "fe5465fe-fe5f-fe5e-fe56-fe6fe1e32ee3"]
pub struct RoomDataDefinition {
    pub house: Vec<RoomFeatureData>,
    pub hotel: Vec<RoomFeatureData>,
    pub shop: Vec<RoomFeatureData>,
    pub clinic: Vec<RoomFeatureData>,
    pub mechanic: Vec<RoomFeatureData>,
    pub gunshop: Vec<RoomFeatureData>,
}
