// To describe how the Pickupablev1 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{
    entity_factory::{
        entities::global::proximity::components::ProximityDataComponent,
        factory::data::{GameEntity, GameEntityData, SpawnEntityEvent},
    },
    game_modules::global_event::systems::GlobalEvent,
};

use super::Pickupablev1Entity;

pub struct Pickupablev1Plugin;

impl Plugin for Pickupablev1Plugin {
    fn build(&self, app: &mut App) {
        // app.add_system(pickupablev1_system);
    }
}

pub fn pickupablev1_spawn(mut commands: &mut Commands, spawn_entity_event: &SpawnEntityEvent) {
    let data = &spawn_entity_event.entity_data;

    match data {
        Some(GameEntityData::Pickupablev1 { on_pickup }) => {
            let mut body = commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 1.0),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: spawn_entity_event.position.unwrap_or_default(),
                    rotation: spawn_entity_event.rotation.unwrap_or_default(),
                    ..Default::default()
                },
                ..Default::default()
            });

            // Base entity
            body.insert(Pickupablev1Entity)
                .insert(ProximityDataComponent {
                    id: spawn_entity_event.id,
                    proximity_distance: 10.0,
                    triggers_event: true,
                    trigger_event: on_pickup.clone(),
                    ..Default::default()
                });
        }
        _ => {}
    }

    // Pickupable entity
}
