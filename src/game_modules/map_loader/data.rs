use bevy::prelude::Vec2;
use serde::{Deserialize, Serialize};

// Map Related Structs

#[derive(Serialize, Deserialize, Debug)]
pub struct PointData {
    pub point_type: String,
    pub center: Vec2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildingData {
    pub id: String,
    pub center: Vec2,
    pub width: f32,
    pub height: f32,
    pub radius: f32,
    pub bldg_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LandFeatureData {
    pub id: String,
    pub start: Vec2,
    pub points: Vec<Vec2>,
    pub points_less: Vec<Vec2>,
    pub center: Vec2,
    pub radius: f32,
    pub points_data: Option<Vec<PointData>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapData {
    pub mountain_list_vectorpoints: Vec<LandFeatureData>,
    pub grayroad_list_vectorpoints: Vec<LandFeatureData>,
    pub forest_list_vectorpoints: Vec<LandFeatureData>,
    pub grassfield_list_vectorpoints: Vec<LandFeatureData>,
    pub land_vectorpoints_outline: LandFeatureData,
    pub sand_vectorpoints_outline: LandFeatureData,
    pub buildings: Vec<BuildingData>,
}

// Interior Related Structs

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomFeatureDescriptionData {
    pub width: f32,
    pub height: f32,
    pub center: Vec2,
    pub element_type: String,
    pub room_code: String,
    pub level: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomFeatureData {
    pub walls: Vec<RoomFeatureDescriptionData>,
    pub doors: Vec<RoomFeatureDescriptionData>,
    pub roofs: Vec<RoomFeatureDescriptionData>,
    pub crates: Vec<RoomFeatureDescriptionData>,
    pub pickups: Vec<RoomFeatureDescriptionData>,
    pub enemies: Vec<RoomFeatureDescriptionData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RoomType {
    House,
    Hotel,
    Shop,
    Clinic,
    Mechanic,
    Gunshop,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomData {
    pub house: Vec<RoomFeatureData>,
    pub hotel: Vec<RoomFeatureData>,
    pub shop: Vec<RoomFeatureData>,
    pub clinic: Vec<RoomFeatureData>,
    pub mechanic: Vec<RoomFeatureData>,
    pub gunshop: Vec<RoomFeatureData>,
}
