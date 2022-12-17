// To describe how the Manager component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::entity_factory::{
    entities::{
        inject_entities, playerv1::systems::plaverv1_spawn, playerv2::systems::plaverv2_spawn,
    },
    factory::data::GameEntity,
};

use super::data::SpawnEntityEvent;

pub struct EntityFactoryPlugin;

impl Plugin for EntityFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEntityEvent>()
            .add_startup_system(factory_init_system)
            .add_system(factory_system);

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
            _ => {}
        }
    }
}
