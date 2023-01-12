// To describe how the Blockv4 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::prelude::RigidBody;

use crate::entity_factory::{
    entities::global::{
        collidable::components::CollidableBody,
        despawn::components::DespawnComponent,
        dissapear_proximity::components::DissapearProximityComponent,
        physics_movable::systems::{insert_physics_components, PhysicsFeature},
    },
    factory::data::{GameEntityData, SpawnEntityEvent},
};

use super::RoofV1Entity;

pub struct Roofv1Plugin;

impl Plugin for Roofv1Plugin {
    fn build(&self, app: &mut App) {}
}

pub fn roofv1_spawn(mut body: &mut EntityCommands, spawn_entity_event: &SpawnEntityEvent) {
    let box_size = spawn_entity_event.size.unwrap_or_default();
    body.insert(SpriteBundle {
        sprite: Sprite {
            color: Color::GRAY,
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
    // Base entity
    body.insert(RoofV1Entity);

    body.insert(DissapearProximityComponent);
}
