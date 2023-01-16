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
    systems::{
        map_loader_system, path_loader_system, MapDataResource, PathDataResource, TileDataResource,
    },
};
use bevy::{math::Vec3Swizzles, prelude::*, render::camera::RenderTarget, utils::HashSet};
use bevy_ecs_tilemap::{
    prelude::{TilemapId, TilemapRenderSettings, TilemapTexture, TilemapTileSize},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder, PathBuilder, StrokeMode},
    shapes,
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, RigidBody, Velocity};
use kdtree::distance::squared_euclidean;
use pathfinding::prelude::{astar, bfs, dijkstra};
use rand::Rng;
use rayon::prelude::*;

#[derive(Resource, Default)]
struct BoracayV2MapGlobals {
    bldg_cam_pos: Option<Vec2>,
    mntn_cam_pos: Option<Vec2>,
    frst_cam_pos: Option<Vec2>,
    gsfl_cam_pos: Option<Vec2>,
    road_cam_pos: Option<Vec2>,
    island_tile_cam_pos: Option<Vec2>,
    mountain_tile_cam_pos: Option<Vec2>,
    cement_tile_cam_pos: Option<Vec2>,
}

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<IVec3>,
}

const TILE_SIZE: TilemapTileSize = TilemapTileSize {
    x: 16.0 * MAP_SCALE,
    y: 16.0 * MAP_SCALE,
};
const RANGE: i32 = 8;
// For this example, don't choose too large a chunk size.
const CHUNK_SIZE: UVec2 = UVec2 {
    x: RANGE as u32 * 2,
    y: RANGE as u32 * 2,
};
// Render chunk sizes are set to 4 render chunks per user specified chunk.
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 8,
    y: CHUNK_SIZE.y * 8,
};
const TRIGGER_SPAWN_RADIUS: f32 = 50.0 * MAP_SCALE;
const SPAWN_RADIUS: f32 = TRIGGER_SPAWN_RADIUS * 2. * MAP_SCALE;

// Shit ball park value for now
const TILE_INDEX_PADDING: IVec2 = IVec2::new(18 - 5, 292 - 68);

#[derive(Resource)]
struct TestTargetIndex {
    start: u32,
    end: u32,
    query_id: String,
}

pub struct BoracayV2MapPlugin;

impl Plugin for BoracayV2MapPlugin {
    fn build(&self, app: &mut App) {
        embed_boracyv2_map(app, None);
    }
}

pub fn embed_boracyv2_map(app: &mut App, system_set: Option<(SystemSet, SystemSet, SystemSet)>) {
    app.insert_resource(TilemapRenderSettings {
        render_chunk_size: RENDER_CHUNK_SIZE,
    })
    .insert_resource(ChunkManager::default())
    .insert_resource(BoracayV2MapGlobals::default())
    .insert_resource(TestTargetIndex {
        start: 0,
        end: 10,
        query_id: "".to_string(),
    });

    // if system set is provided, add systems to the system set, else add to app
    if system_set.is_some() {
        let (on_enter, on_update, on_end) = system_set.unwrap();
        app.add_system_set(
            on_enter.with_system(boracayv2_island_spawn_system.after(map_loader_system)),
        )
        .add_system_set(
            on_update
                .with_system(boracayv2_bldg_stream_system)
                .with_system(boracayv2_mountain_stream_system)
                .with_system(stream_island_tiles)
                .with_system(stream_mountain_tiles)
                // .with_system(stream_cement_tiles)
                .with_system(despawn_tiles)
                .with_system(boracayv2_grassfield_stream_system)
                .with_system(boracayv2_forest_stream_system), // .with_system(boracayv2_road_stream_system),
        )
        .add_system_set(on_end);
    } else {
        app.add_startup_system(boracayv2_island_spawn_system.after(map_loader_system))
            .add_system(boracayv2_bldg_stream_system)
            .add_system(boracayv2_mountain_stream_system)
            .add_system(boracayv2_grassfield_stream_system)
            .add_system(boracayv2_forest_stream_system)
            .add_system(boracayv2_road_stream_system);
    }
}

fn boracayv2_bldg_stream_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut scene_global: ResMut<BoracayV2MapGlobals>,
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
        // let distance =`
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
        let building_type = match building.bldg_type.as_str() {
            "home" => RoomType::SafeHouse,
            "loot" => RoomType::House,
            "big" => RoomType::Hotel,
            "market" => RoomType::Shop,
            "clinic" => RoomType::Clinic,
            "mechanic" => RoomType::Mechanic,
            "gunshop" => RoomType::Gunshop,
            _ => RoomType::House,
        };
        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::Roomv1,
            entity_data: Some(GameEntityData::Roomv1 {
                room_type: building_type,
                despawn_data: DespawnComponent {
                    camera_circle: SPAWN_RADIUS * 1.0,
                    bldg_circle: building.radius * MAP_SCALE,
                    id: building.id.clone(),
                },
            }),
            position: Some(
                (building.center.extend(50.0) * MAP_SCALE)
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

fn boracayv2_mountain_stream_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut scene_global: ResMut<BoracayV2MapGlobals>,
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

fn boracayv2_forest_stream_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut scene_global: ResMut<BoracayV2MapGlobals>,
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
        if distance < TRIGGER_SPAWN_RADIUS * 0.45 / 2.0 {
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
                            radius: SPAWN_RADIUS * 0.45,
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
                        entity: GameEntity::Treev2,
                        entity_data: Some(GameEntityData::Treev1 {
                            despawn_data: DespawnComponent {
                                id: tree.id.clone(),
                                bldg_circle: 43.0,
                                camera_circle: SPAWN_RADIUS * 0.45,
                                ..Default::default()
                            },
                            internal_radius_percentage: 0.20,
                        }),
                        position: Some(
                            tree.center
                                .extend(200. + (rand::random::<usize>() % 50) as f32)
                                * Vec3 {
                                    x: 1.0,
                                    y: -1.0,
                                    z: 1.0,
                                }
                                * MAP_SCALE
                                + forest_start,
                        ),
                        size: Some(Vec2::from([50.0, 50.0])),
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
                        entity: GameEntity::Zombiesv2,
                        entity_data: Some(GameEntityData::Zombiesv2 {
                            despawn_data: DespawnComponent {
                                id: tree.id.clone(),
                                bldg_circle: 15.0,
                                camera_circle: SPAWN_RADIUS * 0.45,
                            },
                        }),
                        position: Some(
                            tree.center.extend(10.)
                                * Vec3 {
                                    x: 1.0,
                                    y: -1.0,
                                    z: 20.0,
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

fn boracayv2_grassfield_stream_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut scene_global: ResMut<BoracayV2MapGlobals>,
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
        if distance < TRIGGER_SPAWN_RADIUS * 0.6 / 2.0 {
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
                    radius: SPAWN_RADIUS * 0.6,
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
        let forest_start = forest.start.extend(50.0)
            * Vec3 {
                x: 1.0,
                y: -1.0,
                z: 1.0,
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
                            radius: SPAWN_RADIUS * 0.6,
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
                        entity: GameEntity::Treev2,
                        entity_data: Some(GameEntityData::Treev1 {
                            despawn_data: DespawnComponent {
                                id: tree.id.clone(),
                                bldg_circle: 43.0,
                                camera_circle: SPAWN_RADIUS * 0.6,
                                ..Default::default()
                            },
                            internal_radius_percentage: 0.20,
                        }),
                        position: Some(
                            tree.center
                                .extend(200. + (rand::random::<usize>() % 50) as f32)
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
                        entity: GameEntity::Zombiesv2,
                        entity_data: Some(GameEntityData::Zombiesv2 {
                            despawn_data: DespawnComponent {
                                id: tree.id.clone(),
                                bldg_circle: 15.0,
                                camera_circle: SPAWN_RADIUS * 0.6,
                            },
                        }),
                        position: Some(
                            tree.center.extend(10.)
                                * Vec3 {
                                    x: 1.0,
                                    y: -1.0,
                                    z: 20.0,
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

fn boracayv2_road_stream_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut scene_global: ResMut<BoracayV2MapGlobals>,
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

fn boracayv2_building_spawn_system(
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

fn boracayv2_island_spawn_system(
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

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    // NOTE: rough estimate of the most top left tile that is visible
    camera_pos / tile_size + TILE_INDEX_PADDING
}

fn spawn_island_chunk(
    commands: &mut Commands,
    tile_data_resource: &Res<TileDataResource>,
    asset_server: &Res<AssetServer>,
    chunk_pos: IVec2,
) {
    // Load assets
    let image_handles = vec![
        asset_server.load("image_tileset/grass/07.png"),
        // correct
        asset_server.load("image_tileset/grass/08.png"),
        asset_server.load("image_tileset/grass/09.png"),
        // correct
        asset_server.load("image_tileset/grass/04.png"),
        // correct
        asset_server.load("image_tileset/grass/05.png"),
        // correct
        asset_server.load("image_tileset/grass/06.png"),
        asset_server.load("image_tileset/grass/01.png"),
        // correct
        asset_server.load("image_tileset/grass/02.png"),
        asset_server.load("image_tileset/grass/03.png"),
        // inners
        asset_server.load("image_tileset/grass/12.png"),
        asset_server.load("image_tileset/grass/13.png"),
        asset_server.load("image_tileset/grass/10.png"),
        asset_server.load("image_tileset/grass/11.png"),
        asset_server.load("image_tileset/grass/00.png"),
    ];
    let transparent_index = image_handles.len() as u32 - 1;
    let texture_vec = TilemapTexture::Vector(image_handles);

    // Load tile data
    let tile_data = tile_data_resource.island_tile_data.as_ref().unwrap();
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    // Spawn the elements of the tilemap.
    // println!("Streaming island");
    for x_index in -RANGE..RANGE {
        for y_index in -RANGE..RANGE {
            let x = chunk_pos.x + x_index;
            let y = chunk_pos.y + y_index;
            let tile = tile_data.points.get(&format!("{:04}_{:04}", x, y));
            if tile.is_none() {
                continue;
            }
            // println!("[{:?}] {:04}_{:04}", chunk_pos, x, y);
            let tile = match tile {
                Some(tile_dat) => tile_dat.tile - 1,
                None => 4,
            };
            let tile_pos = TilePos {
                x: (x_index + RANGE) as u32,
                y: (y_index + RANGE) as u32,
            };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        ((chunk_pos.x - TILE_INDEX_PADDING.x) as f32 - RANGE as f32 - 0.8) * TILE_SIZE.x,
        ((chunk_pos.y - TILE_INDEX_PADDING.y) as f32 - RANGE as f32 + 0.3) * TILE_SIZE.y,
        30.0,
    ));
    // println!("{:?}", transform.translation);
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: CHUNK_SIZE.into(),
        storage: tile_storage,
        texture: texture_vec,
        tile_size: TILE_SIZE,
        transform,
        ..Default::default()
    });
}

fn stream_island_tiles(
    mut commands: Commands,
    tile_data_resource: Res<TileDataResource>,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera2d>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut scene_global: ResMut<BoracayV2MapGlobals>,
) {
    if camera_query.iter().len() == 0 {
        return;
    }

    let camera_xy = camera_query.single().translation.xyy().xy();

    // get if camera moved
    if let Some(cam_pos) = scene_global.island_tile_cam_pos {
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
    scene_global.island_tile_cam_pos = Some(camera_xy);
    // println!("Camera moved");

    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        let camera_chunk_pos_idd = IVec3::new(camera_chunk_pos.x, camera_chunk_pos.y, 0);
        // println!("chunk: {:?}", camera_chunk_pos);
        if !chunk_manager.spawned_chunks.contains(&camera_chunk_pos_idd) {
            chunk_manager.spawned_chunks.insert(camera_chunk_pos_idd);
            spawn_island_chunk(
                &mut commands,
                &tile_data_resource,
                &asset_server,
                camera_chunk_pos,
            );
            // println!("spawned chunk: {:?}", IVec2::new(x, y));
        }
    }
}

fn spawn_mountain_and_cement_chunk(
    commands: &mut Commands,
    tile_data_resource: &Res<TileDataResource>,
    asset_server: &Res<AssetServer>,
    chunk_pos: IVec2,
) {
    // Load assets

    let image_handles = vec![
        asset_server.load("image_tileset/soil/07.png"),
        // correct
        asset_server.load("image_tileset/soil/08.png"),
        asset_server.load("image_tileset/soil/09.png"),
        // correct
        asset_server.load("image_tileset/soil/04.png"),
        // correct
        asset_server.load("image_tileset/soil/05.png"),
        // correct
        asset_server.load("image_tileset/soil/06.png"),
        asset_server.load("image_tileset/soil/01.png"),
        // correct
        asset_server.load("image_tileset/soil/02.png"),
        asset_server.load("image_tileset/soil/03.png"),
        // inners
        asset_server.load("image_tileset/soil/12.png"),
        asset_server.load("image_tileset/soil/13.png"),
        asset_server.load("image_tileset/soil/10.png"),
        asset_server.load("image_tileset/soil/11.png"),
        asset_server.load("image_tileset/soil/00.png"),
        // cement
        asset_server.load("image_tileset/road/07.png"),
        // correct
        asset_server.load("image_tileset/road/08.png"),
        asset_server.load("image_tileset/road/09.png"),
        // correct
        asset_server.load("image_tileset/road/04.png"),
        // correct
        asset_server.load("image_tileset/road/05.png"),
        // correct
        asset_server.load("image_tileset/road/06.png"),
        asset_server.load("image_tileset/road/01.png"),
        // correct
        asset_server.load("image_tileset/road/02.png"),
        asset_server.load("image_tileset/road/03.png"),
        // inners
        asset_server.load("image_tileset/road/12.png"),
        asset_server.load("image_tileset/road/13.png"),
        asset_server.load("image_tileset/road/10.png"),
        asset_server.load("image_tileset/road/11.png"),
        asset_server.load("image_tileset/road/00.png"),
    ];
    let transparent_index = image_handles.len() as u32 - 1;
    let texture_vec = TilemapTexture::Vector(image_handles);

    // Load tile data
    let mntn_tile_data = tile_data_resource.mountain_tile_data.as_ref().unwrap();
    let cmnt_tile_data = tile_data_resource.cement_tile_data.as_ref().unwrap();
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    // Spawn the elements of the tilemap.
    for x_index in -RANGE..RANGE {
        for y_index in -RANGE..RANGE {
            let x = chunk_pos.x + x_index;
            let y = chunk_pos.y + y_index;
            // let tile = cmnt_tile_data.points.get(&format!("{:04}_{:04}", x, y));
            let mntn_tile = mntn_tile_data.points.get(&format!("{:04}_{:04}", x, y));
            let cmnt_tile = cmnt_tile_data.points.get(&format!("{:04}_{:04}", x, y));
            if mntn_tile.is_some() {
                // println!("[{:?}] {:04}_{:04}", chunk_pos, x, y);
                let tile = match mntn_tile {
                    Some(tile_dat) => tile_dat.tile - 1,
                    None => 4,
                };
                let tile_pos = TilePos {
                    x: (x_index + RANGE) as u32,
                    y: (y_index + RANGE) as u32,
                };
                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(tile),
                        ..Default::default()
                    })
                    .id();
                commands.entity(tilemap_entity).add_child(tile_entity);
                tile_storage.set(&tile_pos, tile_entity);
            } else if cmnt_tile.is_some() {
                // println!("[{:?}] {:04}_{:04}", chunk_pos, x, y);
                let tile = match cmnt_tile {
                    Some(tile_dat) => tile_dat.tile - 1,
                    None => 4,
                };
                let tile_pos = TilePos {
                    x: (x_index + RANGE) as u32,
                    y: (y_index + RANGE) as u32,
                };
                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(tile + 14),
                        ..Default::default()
                    })
                    .id();
                commands.entity(tilemap_entity).add_child(tile_entity);
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        ((chunk_pos.x - TILE_INDEX_PADDING.x) as f32 - RANGE as f32 - 0.8) * TILE_SIZE.x,
        ((chunk_pos.y - TILE_INDEX_PADDING.y) as f32 - RANGE as f32 + 0.3) * TILE_SIZE.y,
        40.0,
    ));
    // println!("{:?}", transform.translation);
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: CHUNK_SIZE.into(),
        storage: tile_storage,
        texture: texture_vec,
        tile_size: TILE_SIZE,
        transform,
        ..Default::default()
    });
}

fn stream_mountain_tiles(
    mut commands: Commands,
    tile_data_resource: Res<TileDataResource>,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera2d>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut scene_global: ResMut<BoracayV2MapGlobals>,
) {
    if camera_query.iter().len() == 0 {
        return;
    }

    let camera_xy = camera_query.single().translation.xyy().xy();

    // get if camera moved
    if let Some(cam_pos) = scene_global.mountain_tile_cam_pos {
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
    scene_global.mountain_tile_cam_pos = Some(camera_xy);
    // println!("Camera moved");

    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        let camera_chunk_pos_idd = IVec3::new(camera_chunk_pos.x, camera_chunk_pos.y, 1);
        // println!("chunk: {:?}", camera_chunk_pos);
        if !chunk_manager.spawned_chunks.contains(&camera_chunk_pos_idd) {
            chunk_manager.spawned_chunks.insert(camera_chunk_pos_idd);
            spawn_mountain_and_cement_chunk(
                &mut commands,
                &tile_data_resource,
                &asset_server,
                camera_chunk_pos,
            );
            // println!("spawned chunk: {:?}", IVec2::new(x, y));
        }
    }
}

fn spawn_cement_chunk(
    commands: &mut Commands,
    tile_data_resource: &Res<TileDataResource>,
    asset_server: &Res<AssetServer>,
    chunk_pos: IVec2,
) {
    // Load assets

    let image_handles = vec![
        asset_server.load("image_tileset/road/07.png"),
        // correct
        asset_server.load("image_tileset/road/08.png"),
        asset_server.load("image_tileset/road/09.png"),
        // correct
        asset_server.load("image_tileset/road/04.png"),
        // correct
        asset_server.load("image_tileset/road/05.png"),
        // correct
        asset_server.load("image_tileset/road/06.png"),
        asset_server.load("image_tileset/road/01.png"),
        // correct
        asset_server.load("image_tileset/road/02.png"),
        asset_server.load("image_tileset/road/03.png"),
        // inners
        asset_server.load("image_tileset/road/12.png"),
        asset_server.load("image_tileset/road/13.png"),
        asset_server.load("image_tileset/road/10.png"),
        asset_server.load("image_tileset/road/11.png"),
        asset_server.load("image_tileset/road/00.png"),
    ];
    let transparent_index = image_handles.len() as u32 - 1;
    let texture_vec = TilemapTexture::Vector(image_handles);

    // Load tile data
    let tile_data = tile_data_resource.cement_tile_data.as_ref().unwrap();
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    // Spawn the elements of the tilemap.
    for x_index in -RANGE..RANGE {
        for y_index in -RANGE..RANGE {
            let x = chunk_pos.x + x_index;
            let y = chunk_pos.y + y_index;
            let tile = tile_data.points.get(&format!("{:04}_{:04}", x, y));
            if tile.is_none() {
                continue;
            }
            // println!("[{:?}] {:04}_{:04}", chunk_pos, x, y);
            let tile = match tile {
                Some(tile_dat) => tile_dat.tile - 1,
                None => 4,
            };
            let tile_pos = TilePos {
                x: (x_index + RANGE) as u32,
                y: (y_index + RANGE) as u32,
            };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        ((chunk_pos.x - TILE_INDEX_PADDING.x) as f32 - RANGE as f32 - 0.8) * TILE_SIZE.x,
        ((chunk_pos.y - TILE_INDEX_PADDING.y) as f32 - RANGE as f32 + 0.3) * TILE_SIZE.y,
        35.0,
    ));
    // println!("{:?}", transform.translation);
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: CHUNK_SIZE.into(),
        storage: tile_storage,
        texture: texture_vec,
        tile_size: TILE_SIZE,
        transform,
        ..Default::default()
    });
}

fn stream_cement_tiles(
    mut commands: Commands,
    tile_data_resource: Res<TileDataResource>,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera2d>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut scene_global: ResMut<BoracayV2MapGlobals>,
) {
    if camera_query.iter().len() == 0 {
        return;
    }

    let camera_xy = camera_query.single().translation.xyy().xy();

    // get if camera moved
    if let Some(cam_pos) = scene_global.cement_tile_cam_pos {
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
    scene_global.cement_tile_cam_pos = Some(camera_xy);
    // println!("Camera moved");

    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        let camera_chunk_pos_idd = IVec3::new(camera_chunk_pos.x, camera_chunk_pos.y, 1);
        // println!("chunk: {:?}", camera_chunk_pos);
        if !chunk_manager.spawned_chunks.contains(&camera_chunk_pos_idd) {
            chunk_manager.spawned_chunks.insert(camera_chunk_pos_idd);
            spawn_cement_chunk(
                &mut commands,
                &tile_data_resource,
                &asset_server,
                camera_chunk_pos,
            );
            // println!("spawned chunk: {:?}", IVec2::new(x, y));
        }
    }
}

fn despawn_tiles(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera2d>>,
    chunks_query: Query<(Entity, &Transform), With<TileStorage>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    if false {
        return;
    }
    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > SPAWN_RADIUS * 1.15 {
                let x = (chunk_pos.x as f32 / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y as f32 / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec3::new(x, y, 0));
                chunk_manager.spawned_chunks.remove(&IVec3::new(x, y, 1));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn old_boracayv2_points_spawn_system(
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
    // println!("doing a scan");

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

            // println!("Nearest point [player]: {:?}", np);
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
