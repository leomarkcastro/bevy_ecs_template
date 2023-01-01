// To describe how the Manager component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::entity_factory::{
    entities::{
        blockv1::systems::blockv1_spawn, blockv2::systems::blockv2_spawn,
        blockv3::systems::blockv3_spawn, inject_entities,
        pickupablev1::systems::pickupablev1_spawn, playerv1::systems::plaverv1_spawn,
        playerv2::systems::plaverv2_spawn, polygonv1::systems::polygonv1_spawn,
        polygonv2::systems::polygonv2_spawn, room::systems::roomv1_spawn,
        ui::screen::simple_text::systems::simple_text_spawn, zombiesv1::systems::zombiesv1_spawn,
    },
    factory::data::{GameEntity, UIEntity},
};

use super::data::{SpawnEntityEvent, SpawnUIEvent};

pub struct EntityFactoryPlugin;

impl Plugin for EntityFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEntityEvent>()
            .add_event::<SpawnUIEvent>()
            .add_startup_system(factory_init_system)
            .add_system(factory_system)
            .add_system(ui_factory_system);

        inject_entities(app);
    }
}

fn factory_init_system() {}

fn factory_system(mut commands: Commands, mut spawn_entity_events: EventReader<SpawnEntityEvent>) {
    use GameEntity::*;
    for event in spawn_entity_events.iter() {
        match event.entity {
            TestBox => {
                //generate random color
                let color = Color::rgb(
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                );
                // generate random size
                let size = Vec2::new(rand::random::<f32>() * 100.0, rand::random::<f32>() * 100.0);
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: color,
                        custom_size: Some(size),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: event.position.unwrap_or_default(),
                        rotation: event.rotation.unwrap_or_default(),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
            PlayerV1 => plaverv1_spawn(&mut commands, event),
            PlayerV2 => plaverv2_spawn(&mut commands, event),
            Zombiesv1 => zombiesv1_spawn(&mut commands, event),
            Pickupablev1 => pickupablev1_spawn(&mut commands, event),
            Blockv1 => blockv1_spawn(&mut commands, event),
            Blockv2 => blockv2_spawn(&mut commands, event),
            Blockv3 => blockv3_spawn(&mut commands, event),
            Polygonv1 => polygonv1_spawn(&mut commands, event),
            Polygonv2 => polygonv2_spawn(&mut commands, event),
            Roomv1 => roomv1_spawn(&mut commands, event),
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
