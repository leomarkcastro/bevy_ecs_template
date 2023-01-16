// To describe how the Treev2BaseEntity component/entity should behave.
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

use super::Treev2BaseEntity;

pub struct Treev2BaseEntityPlugin;

impl Plugin for Treev2BaseEntityPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn treev2base_spawn(
    mut body: &mut EntityCommands,
    spawn_entity_event: &SpawnEntityEvent,
    assets_server: &Res<AssetServer>,
) {
    let mut box_size = spawn_entity_event.size.unwrap_or_default();
    // set box size to either x or y, whichever is bigger
    box_size = Vec2::new(box_size.x.max(box_size.y), box_size.x.max(box_size.y));
    body.insert(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(box_size),
            ..Default::default()
        },
        texture: assets_server.load("image_world/tree_stump.png"),
        transform: Transform {
            translation: spawn_entity_event.position.unwrap_or_default(),
            rotation: spawn_entity_event.rotation.unwrap_or_default(),
            ..Default::default()
        },
        ..Default::default()
    });
    // Base entity
    body.insert(Treev2BaseEntity);

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
