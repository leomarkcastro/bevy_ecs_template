// To describe how the Pickupablev1 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::RigidBody;

use crate::{
    entity_factory::{
        entities::global::{
            collidable::components::CollidableBody,
            physics_movable::systems::{insert_physics_components, PhysicsFeature},
            proximity::components::ProximityDataComponent,
        },
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

pub fn pickupablev1_spawn(mut body: &mut EntityCommands, spawn_entity_event: &SpawnEntityEvent) {
    let data = &spawn_entity_event.entity_data;

    match data {
        Some(GameEntityData::Pickupablev1 { on_pickup }) => {
            let box_size = spawn_entity_event.size.unwrap_or_default();
            body.insert(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 1.0),
                    custom_size: Some(box_size),
                    ..Default::default()
                },
                transform: Transform {
                    translation: spawn_entity_event.position.unwrap_or_default(),
                    rotation: spawn_entity_event.rotation.unwrap_or_default(),
                    ..Default::default()
                },
                ..Default::default()
            });

            // get the hypotenuse of width and height
            let hypotenuse = (box_size.x.powi(2) + box_size.y.powi(2)).sqrt();

            // Base entity
            body.insert(Pickupablev1Entity)
                .insert(ProximityDataComponent {
                    id: spawn_entity_event.id,
                    proximity_distance: hypotenuse,
                    triggers_event: true,
                    trigger_event: on_pickup.clone(),
                    ..Default::default()
                });
        }
        _ => {}
    }

    // Pickupable entity
}
