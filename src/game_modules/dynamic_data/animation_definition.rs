use bevy::{
    prelude::{Component, Handle, Resource},
    reflect::TypeUuid,
    utils::HashMap,
};

// To be used as data for the dynamic_data entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(serde::Deserialize)]
pub struct AnimationDefinitionData {
    pub max_width: f32,
    pub max_height: f32,
    pub total_sprites: u32,
    pub aspect_ratio: f32,
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "fe5465fe-fe5f-fe5e-fe56-fe62f1e32fe3"]
pub struct AnimationDefinition {
    pub metadata: HashMap<String, AnimationDefinitionData>,
    pub animation_keys: Vec<String>,
    pub max_width: f32,
    pub max_height: f32,
}
