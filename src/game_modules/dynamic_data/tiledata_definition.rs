use bevy::{
    prelude::{Component, Handle, Resource, Vec2},
    reflect::TypeUuid,
    utils::HashMap,
};

// To be used as data for the dynamic_data entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(serde::Deserialize, Debug)]
pub struct TileDataCell {
    pub x: f32,
    pub y: f32,
    pub tile: u32,
}

#[derive(serde::Deserialize, Debug, TypeUuid)]
#[uuid = "fe54fffe-fe5f-fe5e-fe56-fe62f1e32ee3"]
pub struct TileDataDefinition {
    pub xsize: u32,
    pub ysize: u32,
    pub total: u32,
    pub points: HashMap<String, TileDataCell>,
}
