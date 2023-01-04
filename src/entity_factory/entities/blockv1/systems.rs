// To describe how the Blockv1 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::prelude::RigidBody;

use crate::entity_factory::{
    entities::global::{
        collidable::components::CollidableBody,
        despawn_on_clock::components::DespawnWithTimerComponent,
        physics_movable::systems::{insert_physics_components, PhysicsFeature},
    },
    factory::data::{GameEntityData, SpawnEntityEvent},
};

use super::Blockv1Entity;

pub struct Blockv1Plugin;

impl Plugin for Blockv1Plugin {
    fn build(&self, app: &mut App) {}
}

pub fn old_blockv1_spawn(mut commands: &mut Commands, spawn_entity_event: &SpawnEntityEvent) {
    let box_size = spawn_entity_event.size.unwrap_or_default();
    let mut body = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 1.0, 0.0),
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
    body.insert(Blockv1Entity);

    // Physics
    insert_physics_components(
        &mut body,
        PhysicsFeature {
            size: Some(box_size / 2.0),
            body_type: Some(CollidableBody::Block),
            rigidbody_type: Some(RigidBody::Fixed),
            ..Default::default()
        },
    );
}

pub fn blockv1_spawn(mut body: &mut EntityCommands, spawn_entity_event: &SpawnEntityEvent) {
    let box_size = spawn_entity_event.size.unwrap_or_default();
    body.insert(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.0, 1.0, 0.0, 0.3),
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
    body.insert(Blockv1Entity);

    let spawn_data = &spawn_entity_event.entity_data;

    let mut has_physics = true;
    let mut despawn_timer_data: Option<DespawnWithTimerComponent> = None;
    match spawn_data {
        Some(GameEntityData::Block {
            no_physic,
            despawn_timer_data: dtd,
        }) => {
            has_physics = !no_physic;
            despawn_timer_data = Some(dtd.to_owned());
        }
        _ => {}
    };

    if has_physics {
        // Physics
        insert_physics_components(
            &mut body,
            PhysicsFeature {
                size: Some(box_size / 2.0),
                body_type: Some(CollidableBody::Block),
                rigidbody_type: Some(RigidBody::Fixed),
                record_collidability: true,
                ..Default::default()
            },
        );
    }
    match despawn_timer_data {
        Some(dtd) => {
            body.insert(dtd);
        }
        None => {}
    }
}
