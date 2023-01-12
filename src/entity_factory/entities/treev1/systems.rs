// To describe how the Room component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{
    entity_factory::{
        entities::{
            blockv1::systems::blockv1_spawn, blockv2::systems::blockv2_spawn,
            global::despawn::components::DespawnComponent,
            pickupablev1::systems::pickupablev1_spawn, roofv1::systems::roofv1_spawn,
            zombiesv1::systems::zombiesv1_spawn,
        },
        factory::data::{GameEntityData, SpawnEntityEvent},
    },
    game_modules::{
        global_event::systems::GlobalEvent,
        map_loader::{data::RoomType, systems::RoomDataResource},
    },
};

pub struct Treev1Plugin;

impl Plugin for Treev1Plugin {
    fn build(&self, app: &mut App) {}
}

pub fn treev1_spawn(mut body: &mut EntityCommands, spawn_entity_event: &SpawnEntityEvent) {
    let data = spawn_entity_event.entity_data.as_ref();
    match data {
        Some(GameEntityData::Treev1 {
            despawn_data,
            internal_radius_percentage,
        }) => {
            // create the base room entity
            body.insert(SpriteBundle {
                transform: Transform {
                    translation: spawn_entity_event.position.unwrap_or_default(),
                    rotation: spawn_entity_event.rotation.unwrap_or_default(),
                    ..Default::default()
                },
                ..Default::default()
            });

            body.insert(DespawnComponent {
                bldg_circle: despawn_data.bldg_circle,
                camera_circle: despawn_data.camera_circle,
                id: despawn_data.id.clone(),
            });

            // add the room components [walls]
            body.add_children(|parent| {
                let mut child = parent.spawn_empty();
                blockv1_spawn(
                    &mut child,
                    &SpawnEntityEvent {
                        position: Some(Vec3 {
                            x: 0.,
                            y: 0.,
                            z: 0.0,
                        }),
                        size: Some(spawn_entity_event.size.unwrap() * *internal_radius_percentage),
                        ..Default::default()
                    },
                );
            });

            // add the room components [roofs]
            body.add_children(|parent| {
                let mut child = parent.spawn_empty();
                roofv1_spawn(
                    &mut child,
                    &SpawnEntityEvent {
                        position: Some(Vec3 {
                            x: 0.,
                            y: 0.,
                            z: 100.0,
                        }),
                        size: spawn_entity_event.size,
                        ..Default::default()
                    },
                );
            });
        }
        _ => {}
    }
}
