// To describe how the Blockv3 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;
use bevy_rapier2d::prelude::RigidBody;

use crate::entity_factory::{
    entities::global::{
        collidable::components::CollidableBody,
        despawn::components::DespawnComponent,
        physics_movable::systems::{insert_physics_components, PhysicsFeature},
    },
    factory::data::{GameEntityData, SpawnEntityEvent},
};

use super::Blockv3Entity;

pub struct Blockv3Plugin;

impl Plugin for Blockv3Plugin {
    fn build(&self, app: &mut App) {}
}

pub fn blockv3_spawn(mut commands: &mut Commands, spawn_entity_event: &SpawnEntityEvent) {
    let data = spawn_entity_event.entity_data.as_ref();

    match data {
        Some(GameEntityData::Blockv3 { data }) => {
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
            body.insert(Blockv3Entity).insert(DespawnComponent {
                bldg_circle: data.bldg_circle,
                camera_circle: data.camera_circle,
                id: data.id.clone(),
            });

            // Physics
            insert_physics_components(
                &mut body,
                PhysicsFeature {
                    size: Some(box_size / 2.0),
                    body_type: Some(CollidableBody::Block),
                    rigidbody_type: Some(RigidBody::Fixed),
                    ..Default::default()
                },
            )
        }
        _ => {}
    }
}
