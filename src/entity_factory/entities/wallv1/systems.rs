// To describe how the Wallv1 component/entity should behave.
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

use super::Wallv1Entity;

pub struct Wallv1Plugin;

impl Plugin for Wallv1Plugin {
    fn build(&self, app: &mut App) {}
}

pub fn wallv1_spawn(
    mut body: &mut EntityCommands,
    spawn_entity_event: &SpawnEntityEvent,
    assets_server: &Res<AssetServer>,
) {
    let box_size = spawn_entity_event.size.unwrap_or_default();

    let image = if box_size.x > box_size.y {
        "image_world/wall_h_04.png"
    } else {
        "image_world/wall_04.png"
    };
    body.insert(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(box_size),
            ..Default::default()
        },
        texture: assets_server.load(image),
        transform: Transform {
            translation: spawn_entity_event.position.unwrap_or_default(),
            rotation: spawn_entity_event.rotation.unwrap_or_default(),
            ..Default::default()
        },
        ..Default::default()
    });
    // Base entity
    body.insert(Wallv1Entity);

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
