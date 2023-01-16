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

use super::Floorv1Entity;

pub struct Floorv1Plugin;

impl Plugin for Floorv1Plugin {
    fn build(&self, app: &mut App) {}
}

pub fn floorv1_spawn(
    mut body: &mut EntityCommands,
    spawn_entity_event: &SpawnEntityEvent,
    asset_server: &Res<AssetServer>,
) {
    let box_size = spawn_entity_event.size.unwrap_or_default();

    let image = if box_size.x > box_size.y {
        "image_world/wood_h_01.png"
    } else {
        "image_world/wood_01.png"
    };

    body.insert(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(box_size),
            color: Color::rgb(0.25, 0.25, 0.25),
            ..Default::default()
        },
        texture: asset_server.load(image),
        transform: Transform {
            translation: spawn_entity_event.position.unwrap_or_default(),
            rotation: spawn_entity_event.rotation.unwrap_or_default(),
            ..Default::default()
        },
        ..Default::default()
    });
    // Base entity
    body.insert(Floorv1Entity);
}
