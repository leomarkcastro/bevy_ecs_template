// To describe how the Blockv2 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::entity_factory::{
    entities::global::{
        collidable::components::CollidableBody,
        physics_movable::systems::{insert_physics_components, PhysicsFeature},
    },
    factory::data::SpawnEntityEvent,
};

use super::Blockv2Entity;

pub struct Blockv2Plugin;

impl Plugin for Blockv2Plugin {
    fn build(&self, app: &mut App) {}
}

pub fn blockv2_spawn(mut commands: &mut Commands, spawn_entity_event: &SpawnEntityEvent) {
    let mut body = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 1.0, 0.0),
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
    body.insert(Blockv2Entity);

    // Physics
    insert_physics_components(
        &mut body,
        PhysicsFeature {
            size: Some(Vec2::new(5.0, 5.0)),
            body_type: Some(CollidableBody::Block),
            ..Default::default()
        },
    );
}
