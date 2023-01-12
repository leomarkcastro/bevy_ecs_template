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
        factory::data::{GameEntity, GameEntityData, SpawnEntityEvent},
    },
    game_modules::{
        global_event::systems::GlobalEvent,
        map_loader::{
            data::RoomType,
            systems::{MapDataResource, RoomDataResource},
        },
    },
};

use super::RoomV1Entity;

pub struct RoomV1Plugin;

impl Plugin for RoomV1Plugin {
    fn build(&self, app: &mut App) {}
}

pub fn roomv1_spawn(
    mut body: &mut EntityCommands,
    spawn_entity_event: &SpawnEntityEvent,
    mut room_data: &Res<RoomDataResource>,
) {
    let data = spawn_entity_event.entity_data.as_ref();
    let rd = room_data.room_data.as_ref().unwrap();

    let expected_size = spawn_entity_event.size.unwrap_or_default();

    match data {
        Some(GameEntityData::Roomv1 {
            room_type,
            despawn_data,
        }) => match (room_type) {
            RoomType::House => {
                let room_num = rand::random::<usize>() % 3;
                let house_data = &rd.house[room_num];

                let rot_num = rand::random::<usize>() % 2;
                let rotations = [
                    Quat::from_rotation_z(0.0),
                    // Quat::from_rotation_z(std::f32::consts::PI / 2.0),
                    Quat::from_rotation_z(std::f32::consts::PI),
                    // Quat::from_rotation_z(std::f32::consts::PI * 3.0 / 2.0),
                ];

                let base_size = &house_data.size;
                let scale = Vec3 {
                    x: 0.95 * expected_size.x / base_size.x,
                    y: 0.95 * expected_size.y / base_size.y,
                    z: 1.0,
                };

                // create the base room entity
                body.insert(SpriteBundle {
                    transform: Transform {
                        translation: spawn_entity_event.position.unwrap_or_default(),
                        rotation: rotations[rot_num],
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
                    for wall in &house_data.walls {
                        let mut child = parent.spawn_empty();
                        blockv1_spawn(
                            &mut child,
                            &SpawnEntityEvent {
                                position: Some(
                                    Vec3 {
                                        x: wall.center.x - house_data.center.x,
                                        y: wall.center.y - house_data.center.y,
                                        z: 0.0,
                                    } * scale,
                                ),
                                size: Some(
                                    Vec2 {
                                        x: wall.width,
                                        y: wall.height,
                                    } * scale.truncate(),
                                ),
                                ..Default::default()
                            },
                        );
                    }
                });

                if false {
                    // add the room components [doors]
                    body.add_children(|parent| {
                        // add the doors
                        for doors in &house_data.doors {
                            let mut child = parent.spawn_empty();
                            blockv1_spawn(
                                &mut child,
                                &SpawnEntityEvent {
                                    position: Some(
                                        Vec3 {
                                            x: doors.center.x - house_data.center.x,
                                            y: doors.center.y - house_data.center.y,
                                            z: 0.0,
                                        } * scale,
                                    ),
                                    size: Some(
                                        Vec2 {
                                            x: doors.width,
                                            y: doors.height,
                                        } * scale.truncate(),
                                    ),
                                    ..Default::default()
                                },
                            );
                        }
                    });
                }

                // add the room components [roofs]
                body.add_children(|parent| {
                    // add the roofs
                    for roofs in &house_data.roofs {
                        let mut child = parent.spawn_empty();
                        roofv1_spawn(
                            &mut child,
                            &SpawnEntityEvent {
                                position: Some(
                                    Vec3 {
                                        x: roofs.center.x - house_data.center.x,
                                        y: roofs.center.y - house_data.center.y,
                                        z: 100.0,
                                    } * scale,
                                ),
                                size: Some(
                                    Vec2 {
                                        x: roofs.width,
                                        y: roofs.height,
                                    } * scale.truncate()
                                        * 1.1,
                                ),
                                ..Default::default()
                            },
                        );
                    }
                });

                // add the room components [crates]
                body.add_children(|parent| {
                    // add the crates
                    for obj_crate in &house_data.crates {
                        let mut child = parent.spawn_empty();
                        blockv2_spawn(
                            &mut child,
                            &SpawnEntityEvent {
                                position: Some(
                                    Vec3 {
                                        x: obj_crate.center.x - house_data.center.x,
                                        y: obj_crate.center.y - house_data.center.y,
                                        z: 0.0,
                                    } * scale,
                                ),
                                size: Some(
                                    Vec2 {
                                        x: obj_crate.width,
                                        y: obj_crate.height,
                                    } * scale.truncate(),
                                ),
                                ..Default::default()
                            },
                        );
                    }
                });

                // add the room components [pickups]
                body.add_children(|parent| {
                    // add the pickups
                    for pickups in &house_data.pickups {
                        let mut child = parent.spawn_empty();
                        pickupablev1_spawn(
                            &mut child,
                            &SpawnEntityEvent {
                                position: Some(
                                    Vec3 {
                                        x: pickups.center.x - house_data.center.x,
                                        y: pickups.center.y - house_data.center.y,
                                        z: 0.0,
                                    } * scale,
                                ),
                                size: Some(
                                    Vec2 {
                                        x: pickups.width,
                                        y: pickups.height,
                                    } * scale.truncate(),
                                ),
                                entity_data: Some(GameEntityData::Pickupablev1 {
                                    on_pickup: GlobalEvent {
                                        event_data: format!("pickup"),
                                        scene_id: "PRINT".to_string(),
                                    },
                                }),
                                ..Default::default()
                            },
                        );
                    }
                });

                // add the room components [enemies]
                body.add_children(|parent| {
                    // add the pickups
                    for enemies in &house_data.enemies {
                        let mut child = parent.spawn_empty();
                        zombiesv1_spawn(
                            &mut child,
                            &SpawnEntityEvent {
                                position: Some(
                                    Vec3 {
                                        x: enemies.center.x - house_data.center.x,
                                        y: enemies.center.y - house_data.center.y,
                                        z: 20.0,
                                    } * scale,
                                ),
                                size: Some(
                                    Vec2 {
                                        x: enemies.width,
                                        y: enemies.height,
                                    } * scale.truncate(),
                                ),
                                ..Default::default()
                            },
                        );
                    }
                });
            }
            _ => {}
        },
        _ => {}
    }
}
