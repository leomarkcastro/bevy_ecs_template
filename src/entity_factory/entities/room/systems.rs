// To describe how the Room component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::{
    entity_factory::factory::data::{GameEntityData, SpawnEntityEvent},
    game_modules::map_loader::data::RoomType,
};

use super::RoomEntity;

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn roomv1_spawn(mut commands: &mut Commands, spawn_entity_event: &SpawnEntityEvent) {
    let data = spawn_entity_event.entity_data.as_ref();

    match data {
        Some(GameEntityData::Roomv1 { room_type }) => match (room_type) {
            RoomType::House => {}
            _ => {}
        },
        _ => {}
    }
}
