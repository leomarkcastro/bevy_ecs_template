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

use super::Treev2TopEntity;

pub struct Treev2TopPlugin;

impl Plugin for Treev2TopPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn treev2top_spawn(
    mut body: &mut EntityCommands,
    spawn_entity_event: &SpawnEntityEvent,
    asset_server: &Res<AssetServer>,
) {
    let box_size = spawn_entity_event.size.unwrap_or_default();

    let tree_num = rand::random::<usize>() % 4;
    let tree_option = [
        "image_world/tree_top_01.png",
        "image_world/tree_top_02.png",
        "image_world/tree_top_03.png",
        "image_world/tree_top_04.png",
    ];

    body.insert(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(box_size),
            ..Default::default()
        },
        texture: asset_server.load(tree_option[tree_num]),
        transform: Transform {
            translation: spawn_entity_event.position.unwrap_or_default(),
            rotation: spawn_entity_event.rotation.unwrap_or_default(),
            ..Default::default()
        },
        ..Default::default()
    });
    // Base entity
    body.insert(Treev2TopEntity);

    body.insert(DissapearProximityComponent);
}
