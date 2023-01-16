// To describe how the Manager component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

use crate::{
    entity_factory::{
        entities::{
            blockv1::systems::blockv1_spawn, blockv2::systems::blockv2_spawn,
            blockv3::systems::blockv3_spawn, cratev1::systems::cratev1_spawn, inject_entities,
            pickupablev1::systems::pickupablev1_spawn, playerv1::systems::plaverv1_spawn,
            playerv2::systems::plaverv2_spawn, playerv3::systems::plaverv3_spawn,
            polygonv1::systems::polygonv1_spawn, polygonv2::systems::polygonv2_spawn,
            roofv1::systems::roofv1_spawn, roofv2::systems::roofv2_spawn,
            roomv1::systems::roomv1_spawn, treev1::systems::treev1_spawn,
            treev2::systems::treev2_spawn, ui::screen::simple_text::systems::simple_text_spawn,
            wallv1::systems::wallv1_spawn, zombiesv1::systems::zombiesv1_spawn,
            zombiesv2::systems::zombiesv2_spawn,
        },
        factory::data::{GameEntity, UIEntity},
    },
    game_modules::{load_assets::systems::GameTextures, map_loader::systems::RoomDataResource},
};

use super::data::{SpawnEntityEvent, SpawnUIEvent};

pub struct EntityFactoryPlugin;

impl Plugin for EntityFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEntityEvent>()
            .add_event::<SpawnUIEvent>()
            .add_plugin(ShapePlugin)
            .add_startup_system(factory_init_system)
            .add_system(factory_system)
            .add_system(ui_factory_system);

        inject_entities(app);
    }
}

fn factory_init_system() {}

fn factory_system(
    mut commands: Commands,
    mut spawn_entity_events: EventReader<SpawnEntityEvent>,
    room_data: Res<RoomDataResource>,
    asset_server: Res<AssetServer>,
    game_textures: Res<GameTextures>,
) {
    use GameEntity::*;

    for event in spawn_entity_events.iter() {
        let mut basebody = commands.spawn_empty();
        match event.entity {
            PlayerV1 => plaverv1_spawn(&mut basebody, event),
            PlayerV2 => plaverv2_spawn(&mut basebody, event),
            PlayerV3 => plaverv3_spawn(&mut basebody, event, &game_textures),
            Zombiesv1 => zombiesv1_spawn(&mut basebody, event),
            Zombiesv2 => zombiesv2_spawn(&mut basebody, event, &game_textures),
            Pickupablev1 => pickupablev1_spawn(&mut basebody, event),
            Blockv1 => blockv1_spawn(&mut basebody, event),
            Blockv2 => blockv2_spawn(&mut basebody, event),
            Blockv3 => blockv3_spawn(&mut basebody, event),
            Polygonv1 => polygonv1_spawn(&mut basebody, event),
            Polygonv2 => polygonv2_spawn(&mut basebody, event),
            Roomv1 => roomv1_spawn(
                &mut basebody,
                event,
                &room_data,
                &asset_server,
                &game_textures,
            ),
            Roofv1 => roofv1_spawn(&mut basebody, event),
            Roofv2 => roofv2_spawn(&mut basebody, event, &asset_server),
            Treev1 => treev1_spawn(&mut basebody, event),
            Wallv1 => wallv1_spawn(&mut basebody, event, &asset_server),
            Cratev1 => cratev1_spawn(&mut basebody, event, &asset_server),
            Treev2 => treev2_spawn(&mut basebody, event, &asset_server),
            _ => {}
        }
    }
}

fn ui_factory_system(
    mut commands: Commands,
    mut spawn_entity_events: EventReader<SpawnUIEvent>,
    asset_server: Res<AssetServer>,
) {
    use GameEntity::*;
    for event in spawn_entity_events.iter() {
        match event.entity {
            UIEntity::SimpleText => simple_text_spawn(&mut commands, event, &asset_server),
            _ => {}
        }
    }
}
