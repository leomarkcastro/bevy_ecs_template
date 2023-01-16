// To describe how the Cratev1 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::entity_factory::{
    entities::global::{
        collidable::components::CollidableBody,
        physics_movable::systems::{insert_physics_components, PhysicsFeature},
    },
    factory::data::SpawnEntityEvent,
};

use super::Cratev1Entity;

pub struct Cratev1Plugin;

impl Plugin for Cratev1Plugin {
    fn build(&self, app: &mut App) {}
}

pub fn cratev1_spawn(
    mut body: &mut EntityCommands,
    spawn_entity_event: &SpawnEntityEvent,
    asset_server: &Res<AssetServer>,
) {
    let box_size = spawn_entity_event.size.unwrap_or(Vec2 { x: 10.0, y: 10.0 });
    body.insert(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(box_size),
            color: Color::rgb(0.35, 0.35, 0.35),
            ..Default::default()
        },
        texture: asset_server.load("image_world/box_02.png"),
        transform: Transform {
            translation: spawn_entity_event.position.unwrap_or_default(),
            rotation: spawn_entity_event.rotation.unwrap_or_default(),
            ..Default::default()
        },
        ..Default::default()
    });
    // Base entity
    body.insert(Cratev1Entity);

    // Physics
    insert_physics_components(
        &mut body,
        PhysicsFeature {
            size: Some(box_size / 2.0),
            body_type: Some(CollidableBody::Block),
            weight: 5.0,
            ..Default::default()
        },
    );
}
