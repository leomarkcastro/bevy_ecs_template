// To describe how the DynamicData component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use super::{
    animation_definition::AnimationDefinition, mapdata_definition::MapDataDefinition,
    roomdata_definition::RoomDataDefinition, tiledata_definition::TileDataDefinition,
};

pub struct DynamicDataPlugin;

impl Plugin for DynamicDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(JsonAssetPlugin::<AnimationDefinition>::new(&[
            "animation.json",
        ]))
        .add_plugin(JsonAssetPlugin::<MapDataDefinition>::new(&["map.json"]))
        .add_plugin(JsonAssetPlugin::<RoomDataDefinition>::new(&["rooms.json"]))
        .add_plugin(JsonAssetPlugin::<TileDataDefinition>::new(&[
            "tiledata.json",
        ]));
    }
}
