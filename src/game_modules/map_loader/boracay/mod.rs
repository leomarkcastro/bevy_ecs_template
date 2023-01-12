// To describe how the Boracay component/entity should behave.
// WILL: contain pure logic that interacts with the component

use crate::{
    entity_factory::{
        entities::{
            global::{
                despawn::{components::DespawnComponent, systems::DespawnTrackerGlobal},
                despawn_on_clock::components::DespawnWithTimerComponent,
                physics_movable::components::{PXMovableComponent, PXSize},
            },
            playerv2::entities::Playerv2Entity,
        },
        factory::data::{GameEntity, GameEntityData, SpawnEntityEvent},
    },
    game_modules::{
        camera::systems::camera_init_system,
        controllable::components::ControllableResource,
        pan_camera::components::PanOrbitCamera,
        path_finding::components::{GraphPoint, PathFindProcessResource, PathFindQueryEvent},
        timers::components::ThreeSecondTimer,
    },
    utils::{
        check_collide::{check_2circle_collide, CircleCollideData},
        globals::MAP_SCALE,
    },
};

use super::{
    data::RoomType,
    systems::{map_loader_system, path_loader_system, MapDataResource, PathDataResource},
};
use bevy::{math::Vec3Swizzles, prelude::*, render::camera::RenderTarget};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder, PathBuilder, StrokeMode},
    shapes,
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, RigidBody, Velocity};
use kdtree::distance::squared_euclidean;
use pathfinding::prelude::{astar, bfs, dijkstra};
use rand::Rng;
use rayon::prelude::*;

#[derive(Resource)]
struct BoracayMapGlobals {
    bldg_cam_pos: Option<Vec2>,
    mntn_cam_pos: Option<Vec2>,
    frst_cam_pos: Option<Vec2>,
    gsfl_cam_pos: Option<Vec2>,
    road_cam_pos: Option<Vec2>,
}

#[derive(Resource)]
struct TestTargetIndex {
    start: u32,
    end: u32,
    query_id: String,
}

impl Default for BoracayMapGlobals {
    fn default() -> Self {
        Self {
            bldg_cam_pos: None,
            mntn_cam_pos: None,
            frst_cam_pos: None,
            road_cam_pos: None,
            gsfl_cam_pos: None,
        }
    }
}

const TRIGGER_SPAWN_RADIUS: f32 = 50.0 * MAP_SCALE;
const SPAWN_RADIUS: f32 = TRIGGER_SPAWN_RADIUS * 2. * MAP_SCALE;

pub struct BoracayMapPlugin;

impl Plugin for BoracayMapPlugin {
    fn build(&self, app: &mut App) {
        embed_boracy_map(app, None);
    }
}

pub fn embed_boracy_map(app: &mut App, system_set: Option<(SystemSet, SystemSet, SystemSet)>) {
    app.insert_resource(BoracayMapGlobals::default())
        .insert_resource(TestTargetIndex {
            start: 0,
            end: 10,
            query_id: "".to_string(),
        });

    // if system set is provided, add systems to the system set, else add to app
    if system_set.is_some() {
        let (on_enter, on_update, on_end) = system_set.unwrap();
        app.add_system_set(
            on_enter.with_system(boracay_island_spawn_system.after(map_loader_system)),
        )
        .add_system_set(
            on_update
                .with_system(boracay_bldg_stream_system)
                .with_system(boracay_mountain_stream_system)
                .with_system(boracay_grassfield_stream_system)
                .with_system(boracay_forest_stream_system)
                .with_system(boracay_road_stream_system),
        )
        .add_system_set(on_end);
    } else {
        app.add_startup_system(boracay_island_spawn_system.after(map_loader_system))
            // .add_startup_system(boracay_building_spawn_system.after(boracay_island_spawn_system))
            // .add_startup_system(boracay_mountain_spawn_system.after(boracay_island_spawn_system))
            // .add_startup_system(boracay_forest_spawn_system.after(boracay_island_spawn_system))
            // .add_startup_system(boracay_road_spawn_system.after(boracay_island_spawn_system))
            .add_system(boracay_bldg_stream_system)
            .add_system(boracay_mountain_stream_system)
            .add_system(boracay_grassfield_stream_system)
            .add_system(boracay_forest_stream_system)
            .add_system(boracay_road_stream_system);
    }
}

fn boracay_bldg_stream_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut scene_global: ResMut<BoracayMapGlobals>,
    mut despawn_tracker: ResMut<DespawnTrackerGlobal>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    map_data_res: Res<MapDataResource>,
) {
    if camera_query.iter().len() == 0 {
        return;
    }

    let camera_xy = camera_query.single().translation.xyy().xy();

    // get if camera moved
    if let Some(cam_pos) = scene_global.bldg_cam_pos {
        let distance = (cam_pos - camera_xy).length();
        // println!(
        //     "Distance: {} || SR: {} || check",
        //     distance,
        //     SPAWN_RADIUS / 2.0
        // );
        if distance < TRIGGER_SPAWN_RADIUS {
            return;
        }
    }

    // println!(
    //     "Boracay stream system {} -----------------------",
    //     camera_xy
    // );

    scene_global.bldg_cam_pos = Some(camera_xy);

    let map_data = map_data_res.map_data.as_ref().unwrap();

    let skip = vec!["mountain", "forest", "road"];

    let to_spawn = map_data.buildings.par_iter().filter(|building| {
        let bldg_center = building.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE;
        // let distance =
        //     (camera_xy - building.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE).length();
        // println!("Distance: {} || SR: {}", distance, SPAWN_RADIUS);
        // (distance <= SPAWN_RADIUS)

        // Essentially, this will check if the building is within the spawn radius of the camera
        // And it is not in the skip list
        // And it has not been spawned yet
        (check_2circle_collide(
            CircleCollideData {
                center: camera_xy,
                radius: SPAWN_RADIUS,
            },
            CircleCollideData {
                center: bldg_center,
                radius: building.radius * MAP_SCALE,
            },
        )) && (!skip.contains(&building.bldg_type.as_str()))
            && (!despawn_tracker.spawned_id.contains(&building.id))
    });

    let to_loop = to_spawn.collect::<Vec<_>>();

    if to_loop.len() == 0 {
        return;
    }

    for building in &to_loop {
        despawn_tracker.spawned_id.push(building.id.clone());
        // println!("Spawned: {}", building.id);
        // spawn_entity_events.send(SpawnEntityEvent {
        //     entity: GameEntity::Blockv3,
        //     entity_data: Some(GameEntityData::Blockv3 {
        //         data: DespawnComponent {
        //             camera_circle: SPAWN_RADIUS * 1.5,
        //             bldg_circle: building.radius * MAP_SCALE,
        //             id: building.id.clone(),
        //         },
        //     }),
        //     position: Some(
        //         (building.center.extend(0.5) * MAP_SCALE)
        //             * Vec3 {
        //                 x: 1.0,
        //                 y: -1.0,
        //                 z: 1.0,
        //             },
        //     ),
        //     size: Some(Vec2::new(building.width, building.height) * MAP_SCALE),
        //     ..Default::default()
        // });
        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::Roomv1,
            entity_data: Some(GameEntityData::Roomv1 {
                room_type: RoomType::House,
                despawn_data: DespawnComponent {
                    camera_circle: SPAWN_RADIUS * 1.0,
                    bldg_circle: building.radius * MAP_SCALE,
                    id: building.id.clone(),
                },
            }),
            position: Some(
                (building.center.extend(0.5) * MAP_SCALE)
                    * Vec3 {
                        x: 1.0,
                        y: -1.0,
                        z: 1.0,
                    },
            ),
            size: Some(Vec2::new(building.width, building.height) * MAP_SCALE),
            ..Default::default()
        });
    }
    // println!("---------------- Spawned: {}", to_loop.len());
}

fn boracay_mountain_stream_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut scene_global: ResMut<BoracayMapGlobals>,
    mut despawn_tracker: ResMut<DespawnTrackerGlobal>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    map_data_res: Res<MapDataResource>,
) {
    if camera_query.iter().len() == 0 {
        return;
    }

    let camera_xy = camera_query.single().translation.xyy().xy();

    // get if camera moved
    if let Some(cam_pos) = scene_global.mntn_cam_pos {
        let distance = (cam_pos - camera_xy).length();
        // println!(
        //     "Distance: {} || SR: {} || check",
        //     distance,
        //     SPAWN_RADIUS / 2.0
        // );
        if distance < TRIGGER_SPAWN_RADIUS / 2.0 {
            return;
        }
    }

    // println!(
    //     "Boracay stream system {} -----------------------",
    //     camera_xy
    // );

    scene_global.mntn_cam_pos = Some(camera_xy);

    let map_data = map_data_res.map_data.as_ref().unwrap();

    let to_spawn = map_data
        .mountain_list_vectorpoints
        .par_iter()
        .filter(|building| {
            let bldg_center = building.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE;
            // let distance =
            //     (camera_xy - building.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE).length();
            // println!("Distance: {} || SR: {}", distance, SPAWN_RADIUS);
            // (distance <= SPAWN_RADIUS)
            (check_2circle_collide(
                CircleCollideData {
                    center: camera_xy,
                    radius: SPAWN_RADIUS,
                },
                CircleCollideData {
                    center: bldg_center,
                    radius: building.radius * MAP_SCALE,
                },
            )) && (!despawn_tracker.spawned_id.contains(&building.id))
        });

    let to_loop = to_spawn.collect::<Vec<_>>();

    if to_loop.len() == 0 {
        return;
    }

    for mountain in &to_loop {
        despawn_tracker.spawned_id.push(mountain.id.clone());
        let mountain_1_points = mountain
            .points_less
            .clone()
            .into_iter()
            .map(|p| Vec2::new(p.x, -p.y) * MAP_SCALE)
            .collect::<Vec<Vec2>>();

        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::Polygonv2,
            entity_data: Some(GameEntityData::Polygonv2 {
                path: mountain_1_points,
                style: DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GREEN),
                    outline_mode: StrokeMode::new(Color::DARK_GREEN, MAP_SCALE),
                },
                despawn: DespawnComponent {
                    camera_circle: SPAWN_RADIUS * 1.5,
                    bldg_circle: mountain.radius * MAP_SCALE * 1.5,
                    id: mountain.id.clone(),
                },
                is_collidable: true,
            }),
            position: Some(
                mountain.start.extend(1.0)
                    * Vec3 {
                        x: 1.0,
                        y: -1.0,
                        z: 3.0,
                    }
                    * MAP_SCALE,
            ),
            ..Default::default()
        })
    }
    // println!("---------------- Spawned: {}", to_loop.len());
}

fn boracay_forest_stream_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut scene_global: ResMut<BoracayMapGlobals>,
    mut despawn_tracker: ResMut<DespawnTrackerGlobal>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    map_data_res: Res<MapDataResource>,
) {
    if camera_query.iter().len() == 0 {
        return;
    }

    let camera_xy = camera_query.single().translation.xyy().xy();

    // get if camera moved
    if let Some(cam_pos) = scene_global.frst_cam_pos {
        let distance = (cam_pos - camera_xy).length();
        // println!(
        //     "Distance: {} || SR: {} || check",
        //     distance,
        //     SPAWN_RADIUS / 2.0
        // );
        if distance < TRIGGER_SPAWN_RADIUS / 2.0 {
            return;
        }
    }

    // println!(
    //     "Boracay stream system {} -----------------------",
    //     camera_xy
    // );

    scene_global.frst_cam_pos = Some(camera_xy);

    let map_data = map_data_res.map_data.as_ref().unwrap();

    // Get all the forest within the spawn radius
    let to_spawn = map_data
        .forest_list_vectorpoints
        .par_iter()
        .filter(|building| {
            let bldg_center = building.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE;
            // let distance =
            //     (camera_xy - building.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE).length();
            // println!("Distance: {} || SR: {}", distance, SPAWN_RADIUS);
            // (distance <= SPAWN_RADIUS)
            (check_2circle_collide(
                CircleCollideData {
                    center: camera_xy,
                    radius: SPAWN_RADIUS,
                },
                CircleCollideData {
                    center: bldg_center,
                    radius: building.radius * MAP_SCALE,
                },
            ))
        });

    let to_spawn_forest = to_spawn
        .clone()
        .filter(|building| (!despawn_tracker.spawned_id.contains(&building.id)));

    let to_loop_elements = to_spawn.collect::<Vec<_>>();
    let to_loop_outlines = to_spawn_forest.collect::<Vec<_>>();

    // if to_loop_outlines.len() == 0 && to_loop_elements.len() == 0 {
    //     return;
    // }

    // Spin trees
    for forest in &to_loop_elements {
        let forest_start = forest.start.extend(1.0)
            * Vec3 {
                x: 1.0,
                y: -1.0,
                z: 3.0,
            }
            * MAP_SCALE;

        let par_to_spawn_tree =
            forest
                .points_data
                .as_ref()
                .unwrap()
                .par_iter()
                .filter(|point_data| {
                    let bldg_center = point_data.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE
                        + forest_start.truncate();
                    // let distance =
                    //     (camera_xy - building.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE).length();
                    // println!("Distance: {} || SR: {}", distance, SPAWN_RADIUS);
                    // (distance <= SPAWN_RADIUS)
                    (check_2circle_collide(
                        CircleCollideData {
                            center: camera_xy,
                            radius: SPAWN_RADIUS,
                        },
                        CircleCollideData {
                            center: bldg_center,
                            radius: 15.0,
                        },
                    )) && (!despawn_tracker.spawned_id.contains(&point_data.id))
                });
        let to_spawn_tree = par_to_spawn_tree.collect::<Vec<_>>();
        if to_spawn_tree.len() == 0 {
            continue;
        }
        for tree in &to_spawn_tree {
            match tree.point_type.as_str() {
                "tree" => {
                    despawn_tracker.spawned_id.push(tree.id.clone());
                    // Spawn the forest outline
                    spawn_entity_events.send(SpawnEntityEvent {
                        entity: GameEntity::Treev1,
                        entity_data: Some(GameEntityData::Treev1 {
                            despawn_data: DespawnComponent {
                                id: tree.id.clone(),
                                bldg_circle: 43.0,
                                camera_circle: SPAWN_RADIUS,
                                ..Default::default()
                            },
                            internal_radius_percentage: 0.25,
                        }),
                        position: Some(
                            tree.center.extend(200.)
                                * Vec3 {
                                    x: 1.0,
                                    y: -1.0,
                                    z: 1.0,
                                }
                                * MAP_SCALE
                                + forest_start,
                        ),
                        size: Some(Vec2::from([30.0, 30.0])),
                        ..Default::default()
                    })
                }
                "spawn" => {
                    despawn_tracker.spawned_id.push(tree.id.clone());
                    let mut rng = rand::thread_rng();
                    let random_number: f32 = rng.gen();
                    if random_number > 0.5 {
                        continue;
                    }
                    spawn_entity_events.send(SpawnEntityEvent {
                        entity: GameEntity::Zombiesv1,
                        entity_data: Some(GameEntityData::Zombiesv1 {
                            despawn_data: DespawnComponent {
                                id: tree.id.clone(),
                                bldg_circle: 15.0,
                                camera_circle: SPAWN_RADIUS,
                            },
                        }),
                        position: Some(
                            tree.center.extend(10.)
                                * Vec3 {
                                    x: 1.0,
                                    y: -1.0,
                                    z: 1.0,
                                }
                                * MAP_SCALE
                                + forest_start,
                        ),
                        size: Some(Vec2::from([10.0, 10.0])),
                        ..Default::default()
                    })
                }
                _ => {}
            }
        }
    }

    // Spin forest outline
    for forest in &to_loop_outlines {
        despawn_tracker.spawned_id.push(forest.id.clone());
        let mountain_1_points = forest
            .points_less
            .clone()
            .into_iter()
            .map(|p| Vec2::new(p.x, -p.y) * MAP_SCALE)
            .collect::<Vec<Vec2>>();

        // Spawn the forest outline
        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::Polygonv2,
            entity_data: Some(GameEntityData::Polygonv2 {
                path: mountain_1_points,
                style: DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::rgba(0., 1.0, 0., 0.5)),
                    outline_mode: StrokeMode::new(Color::rgba(0., 0.5, 0., 0.25), MAP_SCALE),
                },
                despawn: DespawnComponent {
                    camera_circle: SPAWN_RADIUS * 1.5,
                    bldg_circle: forest.radius * MAP_SCALE * 1.5,
                    id: forest.id.clone(),
                },
                is_collidable: false,
            }),
            position: Some(
                forest.start.extend(1.0)
                    * Vec3 {
                        x: 1.0,
                        y: -1.0,
                        z: 3.0,
                    }
                    * MAP_SCALE,
            ),
            ..Default::default()
        })
    }
}

fn boracay_grassfield_stream_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut scene_global: ResMut<BoracayMapGlobals>,
    mut despawn_tracker: ResMut<DespawnTrackerGlobal>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    map_data_res: Res<MapDataResource>,
) {
    if camera_query.iter().len() == 0 {
        return;
    }

    let camera_xy = camera_query.single().translation.xyy().xy();

    // get if camera moved
    if let Some(cam_pos) = scene_global.gsfl_cam_pos {
        let distance = (cam_pos - camera_xy).length();
        // println!(
        //     "Distance: {} || SR: {} || check",
        //     distance,
        //     SPAWN_RADIUS / 2.0
        // );
        if distance < TRIGGER_SPAWN_RADIUS / 2.0 {
            return;
        }
    }

    // println!(
    //     "Boracay stream system {} -----------------------",
    //     camera_xy
    // );

    scene_global.gsfl_cam_pos = Some(camera_xy);

    let map_data = map_data_res.map_data.as_ref().unwrap();

    // Get all the forest within the spawn radius
    let to_spawn = map_data
        .grassfield_list_vectorpoints
        .par_iter()
        .filter(|building| {
            let bldg_center = building.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE;
            // let distance =
            //     (camera_xy - building.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE).length();
            // println!("Distance: {} || SR: {}", distance, SPAWN_RADIUS);
            // (distance <= SPAWN_RADIUS)
            (check_2circle_collide(
                CircleCollideData {
                    center: camera_xy,
                    radius: SPAWN_RADIUS,
                },
                CircleCollideData {
                    center: bldg_center,
                    radius: building.radius * MAP_SCALE,
                },
            ))
        });

    let to_spawn_forest = to_spawn
        .clone()
        .filter(|building| (!despawn_tracker.spawned_id.contains(&building.id)));

    let to_loop_elements = to_spawn.collect::<Vec<_>>();
    let to_loop_outlines = to_spawn_forest.collect::<Vec<_>>();

    // if to_loop_outlines.len() == 0 && to_loop_elements.len() == 0 {
    //     return;
    // }

    // Spin trees
    for forest in &to_loop_elements {
        let forest_start = forest.start.extend(1.0)
            * Vec3 {
                x: 1.0,
                y: -1.0,
                z: 3.0,
            }
            * MAP_SCALE;

        let par_to_spawn_tree =
            forest
                .points_data
                .as_ref()
                .unwrap()
                .par_iter()
                .filter(|point_data| {
                    let bldg_center = point_data.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE
                        + forest_start.truncate();
                    // let distance =
                    //     (camera_xy - building.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE).length();
                    // println!("Distance: {} || SR: {}", distance, SPAWN_RADIUS);
                    // (distance <= SPAWN_RADIUS)
                    (check_2circle_collide(
                        CircleCollideData {
                            center: camera_xy,
                            radius: SPAWN_RADIUS,
                        },
                        CircleCollideData {
                            center: bldg_center,
                            radius: 15.0,
                        },
                    )) && (!despawn_tracker.spawned_id.contains(&point_data.id))
                });
        let to_spawn_tree = par_to_spawn_tree.collect::<Vec<_>>();
        if to_spawn_tree.len() == 0 {
            continue;
        }
        for tree in &to_spawn_tree {
            match tree.point_type.as_str() {
                "tree" => {
                    despawn_tracker.spawned_id.push(tree.id.clone());
                    // Spawn the forest outline
                    spawn_entity_events.send(SpawnEntityEvent {
                        entity: GameEntity::Treev1,
                        entity_data: Some(GameEntityData::Treev1 {
                            despawn_data: DespawnComponent {
                                id: tree.id.clone(),
                                bldg_circle: 43.0,
                                camera_circle: SPAWN_RADIUS,
                                ..Default::default()
                            },
                            internal_radius_percentage: 0.25,
                        }),
                        position: Some(
                            tree.center.extend(200.)
                                * Vec3 {
                                    x: 1.0,
                                    y: -1.0,
                                    z: 1.0,
                                }
                                * MAP_SCALE
                                + forest_start,
                        ),
                        size: Some(Vec2::from([30.0, 30.0])),
                        ..Default::default()
                    })
                }
                "spawn" => {
                    despawn_tracker.spawned_id.push(tree.id.clone());
                    let mut rng = rand::thread_rng();
                    let random_number: f32 = rng.gen();
                    if random_number > 0.5 {
                        continue;
                    }
                    spawn_entity_events.send(SpawnEntityEvent {
                        entity: GameEntity::Zombiesv1,
                        entity_data: Some(GameEntityData::Zombiesv1 {
                            despawn_data: DespawnComponent {
                                id: tree.id.clone(),
                                bldg_circle: 15.0,
                                camera_circle: SPAWN_RADIUS,
                            },
                        }),
                        position: Some(
                            tree.center.extend(10.)
                                * Vec3 {
                                    x: 1.0,
                                    y: -1.0,
                                    z: 1.0,
                                }
                                * MAP_SCALE
                                + forest_start,
                        ),
                        size: Some(Vec2::from([10.0, 10.0])),
                        ..Default::default()
                    })
                }
                _ => {}
            }
        }
    }

    // Spin forest outline
    for forest in &to_loop_outlines {
        despawn_tracker.spawned_id.push(forest.id.clone());
        let mountain_1_points = forest
            .points_less
            .clone()
            .into_iter()
            .map(|p| Vec2::new(p.x, -p.y) * MAP_SCALE)
            .collect::<Vec<Vec2>>();

        // Spawn the forest outline
        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::Polygonv2,
            entity_data: Some(GameEntityData::Polygonv2 {
                path: mountain_1_points,
                style: DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::rgba(0., 1.0, 0., 0.05)),
                    outline_mode: StrokeMode::new(Color::rgba(0., 0.5, 0., 0.025), MAP_SCALE),
                },
                despawn: DespawnComponent {
                    camera_circle: SPAWN_RADIUS * 1.5,
                    bldg_circle: forest.radius * MAP_SCALE * 1.5,
                    id: forest.id.clone(),
                },
                is_collidable: false,
            }),
            position: Some(
                forest.start.extend(1.0)
                    * Vec3 {
                        x: 1.0,
                        y: -1.0,
                        z: 3.0,
                    }
                    * MAP_SCALE,
            ),
            ..Default::default()
        })
    }
}

fn boracay_road_stream_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut scene_global: ResMut<BoracayMapGlobals>,
    mut despawn_tracker: ResMut<DespawnTrackerGlobal>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    map_data_res: Res<MapDataResource>,
) {
    if camera_query.iter().len() == 0 {
        return;
    }

    let camera_xy = camera_query.single().translation.xyy().xy();

    // get if camera moved
    if let Some(cam_pos) = scene_global.road_cam_pos {
        let distance = (cam_pos - camera_xy).length();
        // println!(
        //     "Distance: {} || SR: {} || check",
        //     distance,
        //     SPAWN_RADIUS / 2.0
        // );
        if distance < TRIGGER_SPAWN_RADIUS / 2.0 {
            return;
        }
    }

    // println!(
    //     "Boracay stream system {} -----------------------",
    //     camera_xy
    // );

    scene_global.road_cam_pos = Some(camera_xy);

    let map_data = map_data_res.map_data.as_ref().unwrap();

    let to_spawn = map_data
        .grayroad_list_vectorpoints
        .par_iter()
        .filter(|building| {
            let bldg_center = building.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE;
            // let distance =
            //     (camera_xy - building.center * Vec2 { x: 1.0, y: -1.0 } * MAP_SCALE).length();
            // println!("Distance: {} || SR: {}", distance, SPAWN_RADIUS);
            // (distance <= SPAWN_RADIUS)
            (check_2circle_collide(
                CircleCollideData {
                    center: camera_xy,
                    radius: SPAWN_RADIUS * 2.0,
                },
                CircleCollideData {
                    center: bldg_center,
                    radius: building.radius * MAP_SCALE,
                },
            )) && (!despawn_tracker.spawned_id.contains(&building.id))
        });

    let to_loop = to_spawn.collect::<Vec<_>>();

    if to_loop.len() == 0 {
        return;
    }

    for road in &to_loop {
        despawn_tracker.spawned_id.push(road.id.clone());
        let mountain_1_points = road
            .points_less
            .clone()
            .into_iter()
            .map(|p| Vec2::new(p.x, -p.y) * MAP_SCALE)
            .collect::<Vec<Vec2>>();

        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::Polygonv2,
            entity_data: Some(GameEntityData::Polygonv2 {
                path: mountain_1_points,
                style: DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::rgb(0.2, 0.2, 0.2)),
                    outline_mode: StrokeMode::new(Color::rgb(0.2, 0.2, 0.2), MAP_SCALE * 10.),
                },
                despawn: DespawnComponent {
                    camera_circle: SPAWN_RADIUS * 2.0 * 1.5,
                    bldg_circle: road.radius * MAP_SCALE,
                    id: road.id.clone(),
                },
                is_collidable: false,
            }),
            position: Some(
                road.start.extend(1.0)
                    * Vec3 {
                        x: 1.0,
                        y: -1.0,
                        z: 3.0,
                    }
                    * MAP_SCALE,
            ),
            ..Default::default()
        })
    }
    // println!("---------------- Spawned: {}", to_loop.len());
}

fn boracay_building_spawn_system(
    mut commands: Commands,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    map_data: Res<MapDataResource>,
) {
    println!("Boracay building spawn");
    let map_data = map_data.map_data.as_ref().unwrap();

    // to skip list
    let skip = vec!["mountain", "forest", "road"];

    for building in &map_data.buildings {
        if skip.contains(&building.bldg_type.as_str()) {
            continue;
        }
        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::Blockv1,
            position: Some(
                (building.center.extend(0.5) * MAP_SCALE)
                    * Vec3 {
                        x: 1.0,
                        y: -1.0,
                        z: 1.0,
                    },
            ),
            size: Some(Vec2::new(building.width, building.height) * MAP_SCALE),
            ..Default::default()
        });
    }
}

fn boracay_island_spawn_system(
    mut commands: Commands,
    map_data: Res<MapDataResource>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
) {
    println!("Boracay island spawn");

    let map_data = map_data.map_data.as_ref().unwrap();

    let island = &map_data.land_vectorpoints_outline;

    let mountain_1_points = island
        .points_less
        .clone()
        .into_iter()
        .map(|p| Vec2::new(p.x, -p.y) * MAP_SCALE)
        .collect::<Vec<Vec2>>();

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(mountain_1_points[0]);
    for pts in mountain_1_points.iter().skip(1) {
        path_builder.line_to(Vec2::new(pts.x, pts.y));
    }
    let line = path_builder.build();

    // crete [0,1,2 ... n] index for mountain_1_points
    let mut index = 0;
    let mut index_list = Vec::new();
    for _ in mountain_1_points.iter().skip(1) {
        index_list.push([index, index + 1]);
        index += 1;
    }

    let island_shape = GeometryBuilder::build_as(
        &line,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::hex("567D46").unwrap()),
            outline_mode: StrokeMode::new(Color::hex("7EC850").unwrap(), MAP_SCALE),
        },
        Transform {
            translation: island.start.extend(0.0)
                * Vec3 {
                    x: 1.0,
                    y: -1.0,
                    z: 1.0,
                }
                * MAP_SCALE,
            ..Default::default()
        },
    );

    commands
        .spawn(island_shape)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(RigidBody::Fixed)
        .insert(Velocity::zero())
        .insert(Collider::polyline(mountain_1_points, Some(index_list)));
    // .insert(Collider::convex_decomposition(
    //     mountain_1_points.as_slice(),
    //     index_list.as_slice(),
    // ));
}

fn old_boracay_points_spawn_system(
    path_data: Res<PathDataResource>,
    target_data: Res<TestTargetIndex>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    time: Res<Time>,
    mut three_sec_timer: ResMut<ThreeSecondTimer>,
    collidables: Query<(&PXSize, &GlobalTransform)>,
    player_query: Query<(&PXSize, &GlobalTransform), With<Playerv2Entity>>,
) {
    if !three_sec_timer.event_timer.tick(time.delta()).finished() {
        return;
    }
    println!("doing a scan");

    let map_data = path_data.path_data.as_ref().unwrap();
    let points = &map_data.points;
    let vertices = &map_data.vertices;
    let point_length = points.len();

    // generate random number
    // let mut rng = rand::thread_rng();
    // let random_number_a = rng.gen_range(0..point_length as u32);
    // let random_number_b = rng.gen_range(0..point_length as u32);
    // let random_number_a = point_length as u32 - 1;
    let mut random_number_a = target_data.start;
    // let random_number_b = point_length as u32 - 7000;
    let mut random_number_b = target_data.end;

    if let Ok((_, pl_gtransform)) = player_query.get_single() {
        let nearest_point = path_data.kdtree.as_ref();

        if nearest_point.is_some() {
            let min = pl_gtransform.to_scale_rotation_translation().2.truncate();
            let np = nearest_point
                .unwrap()
                .nearest(
                    &[min.x / MAP_SCALE, min.y / MAP_SCALE],
                    1,
                    &squared_euclidean,
                )
                .unwrap_or_default();

            random_number_a = np[0].1.to_owned() as u32;

            println!("Nearest point [player]: {:?}", np);
        }
    }

    random_number_a = 0;
    random_number_b = 10000;

    // get the first player position

    let start: GraphPoint = GraphPoint(random_number_a as u32);
    let goal: GraphPoint = GraphPoint(random_number_b as u32);

    // get iterable of collidables
    let collidables = collidables.iter().collect::<Vec<_>>();

    // get the box of the collidables
    let collidables_box = collidables
        .iter()
        .map(|(collider, gtransform)| {
            // get the translation, scale of globaltransform

            let min = gtransform.to_scale_rotation_translation().2.truncate() / MAP_SCALE;
            let rotation_quat = gtransform.to_scale_rotation_translation().1;

            let max = Vec2::from((collider.width, collider.height)) / (2.);
            // rotate the collider, get the rotation Vec2

            let target_asset_angle =
                rotation_quat.to_axis_angle().1 * rotation_quat.to_axis_angle().0.z;
            // convert f32 to vec2
            let rotation = Vec2::new(target_asset_angle.cos(), target_asset_angle.sin());

            let collider = max.rotate(rotation);
            (min, max)
        })
        .collect::<Vec<_>>();

    /*
    let result_a = bfs(&start, |p| p.successors(map_data), |p| *p == goal);

    match result_a {
        Some(path) => {
            for point in path {
                let loc = point_locaitons[point.0 as usize];
                spawn_entity_events.send(SpawnEntityEvent {
                    entity: GameEntity::Blockv1,
                    entity_data: Some(GameEntityData::Block { no_physic: true }),
                    position: Some(
                        (loc.extend(0.5) * MAP_SCALE)
                            * Vec3 {
                                x: 1.0,
                                y: 1.0,
                                z: 100.0,
                            },
                    ),
                    size: Some(Vec2::new(5.0, 5.0) * MAP_SCALE),
                    ..Default::default()
                });
            }
        }
        None => println!("No path found"),
    }

    */
    if start != goal {
        let result_b = astar(
            &start,
            |p| p.successors_weighted_collissioned(points, vertices, &collidables_box),
            |p| p.distance(points, random_number_b as u32) as usize,
            |p| *p == goal,
        );

        match result_b {
            Some((path, len)) => {
                for point in path {
                    let loc = points[point.0 as usize];
                    let current_time = time.elapsed_seconds();
                    // get current time after 5 seconds
                    let despawn_time = current_time + 2.0;
                    spawn_entity_events.send(SpawnEntityEvent {
                        entity: GameEntity::Blockv1,
                        entity_data: Some(GameEntityData::Block {
                            no_physic: true,
                            despawn_timer_data: DespawnWithTimerComponent {
                                despawn_on: despawn_time,
                                ..Default::default()
                            },
                        }),
                        position: Some(
                            (loc.extend(0.5) * MAP_SCALE)
                                * Vec3 {
                                    x: 1.0,
                                    y: 1.0,
                                    z: 100.0,
                                },
                        ),
                        size: Some(Vec2::new(5.0, 5.0)),
                        ..Default::default()
                    });
                }
                println!("path length: {}", len);
            }
            None => println!("No path found"),
        }
    }

    // let shape = shapes::Circle {
    //     radius: 5.0,
    //     ..Default::default()
    // };

    // for point in path_points {
    //     spawn_entity_events.send(SpawnEntityEvent {
    //         entity: GameEntity::Blockv1,
    //         entity_data: Some(GameEntityData::Block { no_physic: true }),
    //         position: Some(
    //             (point.extend(0.5) * MAP_SCALE)
    //                 * Vec3 {
    //                     x: 1.0,
    //                     y: 1.0,
    //                     z: 100.0,
    //                 },
    //         ),
    //         size: Some(Vec2::new(2.0, 2.0) * MAP_SCALE),
    //         ..Default::default()
    //     });
    // }
    // .insert(Collider::convex_decomposition(
    //     mountain_1_points.as_slice(),
    //     index_list.as_slice(),
    // ));
}

/*
fn boracay_points_spawn_system(
    time: Res<Time>,
    path_data: Res<PathDataResource>,
    player_query: Query<(&GlobalTransform), With<Playerv2Entity>>,
    mut three_sec_timer: ResMut<ThreeSecondTimer>,
    mut pathfinding_process: ResMut<PathFindProcessResource>,
    mut pathfind_query: EventWriter<PathFindQueryEvent>,
    mut target_data: ResMut<TestTargetIndex>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
) {
    let map_data = path_data.path_data.as_ref().unwrap();
    let points = &map_data.points;

    if target_data.query_id.len() != 0 {
        if !pathfinding_process
            .processess
            .contains_key(&target_data.query_id)
        {
            println!("Process not found");
            target_data.query_id = "".to_string();
            return;
        } else {
            let process = pathfinding_process
                .processess
                .get(&target_data.query_id)
                .unwrap();
            if (process.task_buffer.is_some()) {
                if !three_sec_timer.event_timer.tick(time.delta()).finished() {
                    return;
                }
            } else {
                match &process.path {
                    Some(proc_path) => {
                        let path = process;
                        // println!(
                        //     "{:?} -> {:?} -> {:?}",
                        //     process.start,
                        //     proc_path.len(),
                        //     process.goal
                        // );
                        for point in proc_path {
                            let loc = points[point.0 as usize];
                            let current_time = time.elapsed_seconds();
                            // get current time after 5 seconds
                            let despawn_time = current_time + 2.0;
                            spawn_entity_events.send(SpawnEntityEvent {
                                entity: GameEntity::Blockv1,
                                entity_data: Some(GameEntityData::Block {
                                    no_physic: true,
                                    despawn_timer_data: DespawnWithTimerComponent {
                                        despawn_on: despawn_time,
                                        ..Default::default()
                                    },
                                }),
                                position: Some(
                                    (loc.extend(0.5) * MAP_SCALE)
                                        * Vec3 {
                                            x: 1.0,
                                            y: 1.0,
                                            z: 100.0,
                                        },
                                ),
                                size: Some(Vec2::new(7.5, 7.5)),
                                ..Default::default()
                            });
                        }
                    }
                    None => println!("not done yet"),
                }
                target_data.query_id = "".to_string();
            }
        }
    } else {
        if !three_sec_timer.event_timer.tick(time.delta()).finished() {
            return;
        }
        println!("doing a scan ({})", target_data.query_id);

        let mut random_number_a = 0;
        let random_number_b = target_data.end;

        if let Ok((pl_gtransform)) = player_query.get_single() {
            let nearest_point = path_data.kdtree.as_ref();

            if nearest_point.is_some() {
                let min = pl_gtransform.to_scale_rotation_translation().2.truncate();
                let np = nearest_point
                    .unwrap()
                    .nearest(
                        &[min.x / MAP_SCALE, min.y / MAP_SCALE],
                        1,
                        &squared_euclidean,
                    )
                    .unwrap_or_default();

                random_number_a = np[0].1.to_owned() as u32;

                println!("Nearest point [player]: {:?}", np);
            }
        }

        let query = PathFindQueryEvent {
            start: random_number_a,
            goal: random_number_b,
            ..Default::default()
        };
        target_data.query_id = query.id.to_string();
        pathfind_query.send(query);
    }
}


fn boracay_mountain_spawn_system(mut commands: Commands, map_data: Res<MapDataResource>) {
    println!("Boracay mountain spawn");

    let map_data = map_data.map_data.as_ref().unwrap();

    for mountain in &map_data.mountain_list_vectorpoints {
        let mountain_1_points = mountain
            .points_less
            .clone()
            .into_iter()
            .map(|p| Vec2::new(p.x, -p.y) * MAP_SCALE)
            .collect::<Vec<Vec2>>();

        let mut path_builder = PathBuilder::new();
        path_builder.move_to(mountain_1_points[0]);
        for pts in mountain_1_points.iter().skip(1) {
            path_builder.line_to(Vec2::new(pts.x, pts.y));
        }
        let line = path_builder.build();

        let mut index = 0;
        let mut index_list = Vec::new();
        for _ in mountain_1_points.iter().skip(1) {
            index_list.push([index, index + 1]);
            index += 1;
        }

        commands
            .spawn(GeometryBuilder::build_as(
                &line,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GREEN),
                    outline_mode: StrokeMode::new(Color::DARK_GREEN, MAP_SCALE),
                },
                Transform {
                    translation: mountain.start.extend(1.0)
                        * Vec3 {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        }
                        * MAP_SCALE,
                    ..Default::default()
                },
            ))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(RigidBody::Fixed)
            .insert(Velocity::zero())
            // .insert(Collider::convex_decomposition(
            //     mountain_1_points.as_slice(),
            //     index_list.as_slice(),
            // ))
            .insert(Collider::polyline(mountain_1_points, Some(index_list)));
    }
}

fn boracay_forest_spawn_system(mut commands: Commands, map_data: Res<MapDataResource>) {
    println!("Boracay forest spawn");

    let map_data = map_data.map_data.as_ref().unwrap();

    for forest in &map_data.forest_list_vectorpoints {
        let forest_points = forest
            .points_less
            .clone()
            .into_iter()
            .map(|p| Vec2::new(p.x, -p.y) * MAP_SCALE)
            .collect::<Vec<Vec2>>();

        let mut path_builder = PathBuilder::new();
        path_builder.move_to(forest_points[0]);
        for pts in forest_points.iter().skip(1) {
            path_builder.line_to(Vec2::new(pts.x, pts.y));
        }
        let line = path_builder.build();

        commands.spawn(GeometryBuilder::build_as(
            &line,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::rgba(0., 1.0, 0., 0.5)),
                outline_mode: StrokeMode::new(Color::rgba(0., 0.5, 0., 0.25), MAP_SCALE),
            },
            Transform {
                translation: forest.start.extend(1.0)
                    * Vec3 {
                        x: 1.0,
                        y: -1.0,
                        z: 1.0,
                    }
                    * MAP_SCALE,
                ..Default::default()
            },
        ));
    }
}

fn boracay_road_spawn_system(mut commands: Commands, map_data: Res<MapDataResource>) {
    println!("Boracay road spawn");

    let map_data = map_data.map_data.as_ref().unwrap();

    for road in &map_data.grayroad_list_vectorpoints {
        let road_points = road
            .points_less
            .clone()
            .into_iter()
            .map(|p| Vec2::new(p.x, -p.y) * MAP_SCALE)
            .collect::<Vec<Vec2>>();

        let mut path_builder = PathBuilder::new();
        path_builder.move_to(road_points[0]);
        for pts in road_points.iter().skip(1) {
            path_builder.line_to(Vec2::new(pts.x, pts.y));
        }
        let line = path_builder.build();

        commands.spawn(GeometryBuilder::build_as(
            &line,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::rgb(0.2, 0.2, 0.2)),
                outline_mode: StrokeMode::new(Color::rgb(0.2, 0.2, 0.2), MAP_SCALE * 10.),
            },
            Transform {
                translation: road.start.extend(1.0)
                    * Vec3 {
                        x: 1.0,
                        y: -1.0,
                        z: 1.0,
                    }
                    * MAP_SCALE,
                ..Default::default()
            },
        ));
    }
}

fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    time: Res<Time>,
    controllable_component: Res<ControllableResource>,
    path_data: Res<PathDataResource>,
    mut target_data: ResMut<TestTargetIndex>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    if (controllable_component.btn_a.pressed) {
        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            let nearest_point = path_data.kdtree.as_ref();

            if nearest_point.is_some() {
                let np = nearest_point
                    .unwrap()
                    .nearest(
                        &[world_pos.x / MAP_SCALE, world_pos.y / MAP_SCALE],
                        1,
                        &squared_euclidean,
                    )
                    .unwrap_or_default();

                target_data.end = np[0].1.to_owned() as u32;

                println!("Nearest point [cursor]: {:?}", np);
            } else {
                // let current_time = time.elapsed_seconds();
                // // get current time after 5 seconds
                // let despawn_time = current_time + 20.0;
                // spawn_entity_events.send(SpawnEntityEvent {
                //     entity: GameEntity::Blockv1,
                //     entity_data: Some(GameEntityData::Block {
                //         no_physic: false,
                //         despawn_timer_data: DespawnWithTimerComponent {
                //             despawn_on: despawn_time,
                //             ..Default::default()
                //         },
                //     }),
                //     position: Some(
                //         (world_pos.extend(0.5))
                //             * Vec3 {
                //                 x: 1.0,
                //                 y: 1.0,
                //                 z: 100.0,
                //             },
                //     ),
                //     size: Some(Vec2::new(80.0, 40.0) * MAP_SCALE),
                //     ..Default::default()
                // });
            }
        }
    }
}

*/
