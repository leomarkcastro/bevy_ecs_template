// To describe how the Zombiesv1 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::entity_factory::{
    entities::global::{
        ai::{
            components::{AIIdentifier, AIMode, AIStatus, AITeam},
            entities::AIEntity,
        },
        collidable::components::CollidableBody,
        despawn::components::DespawnComponent,
        health::components::HealthComponent,
        physics_movable::systems::{insert_physics_components, PhysicsFeature},
    },
    factory::data::{GameEntityData, SpawnEntityEvent},
};

use super::{Zombiesv1Component, Zombiesv1Entity};

pub struct Zombiesv1Plugin;

impl Plugin for Zombiesv1Plugin {
    fn build(&self, app: &mut App) {
        // app.add_system(zombiesv1_system);
    }
}

pub fn zombiesv1_spawn(mut body: &mut EntityCommands, spawn_entity_event: &SpawnEntityEvent) {
    let data = spawn_entity_event.entity_data.as_ref();
    let mut _despawn_data = None;
    if let Some(GameEntityData::Zombiesv1 { despawn_data }) = data {
        _despawn_data = Some(despawn_data.clone());
    }

    body.insert(SpriteBundle {
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
    body.insert(Zombiesv1Entity)
        .insert(HealthComponent::default());

    // Physics
    insert_physics_components(
        &mut body,
        PhysicsFeature {
            size: Some(Vec2::new(5.0, 5.0)),
            body_type: Some(CollidableBody::Enemy),
            ..Default::default()
        },
    );

    // Zombiesv1
    // AI
    body.insert(AIEntity)
        .insert(AIStatus {
            active: true,
            ..Default::default()
        })
        .insert(AIIdentifier {
            team: AITeam::Zombies,
            ..Default::default()
        });

    if _despawn_data.is_some() {
        let dd = _despawn_data.unwrap();
        // Dissapear after sitance
        body.insert(DespawnComponent {
            bldg_circle: dd.bldg_circle,
            camera_circle: dd.camera_circle,
            id: dd.id.clone(),
        });
    }
}
