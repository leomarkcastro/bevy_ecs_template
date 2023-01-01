// To describe how the Boracay component/entity should behave.
// WILL: contain pure logic that interacts with the component

use crate::{
    entity_factory::{
        entities::global::{
            despawn::{components::DespawnComponent, systems::DespawnTrackerGlobal},
            physics_movable::components::PXMovableComponent,
        },
        factory::data::{GameEntity, GameEntityData, SpawnEntityEvent},
    },
    game_modules::camera::systems::camera_init_system,
    utils::check_2circle_collide::{check_2circle_collide, CircleCollideData},
};

use super::systems::{map_knearest, map_loader_system, MapDataResource};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::prelude::{DrawMode, FillMode, GeometryBuilder, PathBuilder, StrokeMode};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, RigidBody, Velocity};
use rayon::prelude::*;

#[derive(Resource)]
struct BoracayMapGlobals {
    bldg_cam_pos: Option<Vec2>,
    mntn_cam_pos: Option<Vec2>,
    frst_cam_pos: Option<Vec2>,
    road_cam_pos: Option<Vec2>,
}

impl Default for BoracayMapGlobals {
    fn default() -> Self {
        Self {
            bldg_cam_pos: None,
            mntn_cam_pos: None,
            frst_cam_pos: None,
            road_cam_pos: None,
        }
    }
}

const MAP_SCALE: f32 = 2.0;
const TRIGGER_SPAWN_RADIUS: f32 = 100.0 * MAP_SCALE;
const SPAWN_RADIUS: f32 = 300.0 * MAP_SCALE;

pub struct BoracayMapPlugin;

impl Plugin for BoracayMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoracayMapGlobals::default())
            .add_startup_system(boracay_island_spawn_system.after(map_loader_system))
            // .add_startup_system(boracay_building_spawn_system.after(boracay_island_spawn_system))
            // .add_startup_system(boracay_mountain_spawn_system.after(boracay_island_spawn_system))
            // .add_startup_system(boracay_forest_spawn_system.after(boracay_island_spawn_system))
            // .add_startup_system(boracay_road_spawn_system.after(boracay_island_spawn_system))
            .add_system(boracay_bldg_stream_system)
            .add_system(boracay_mountain_stream_system)
            .add_system(boracay_forest_stream_system)
            .add_system(boracay_road_stream_system);
    }
}

fn boracay_bldg_stream_system(
    camera_query: Query<&Transform, With<Camera>>,
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
        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::Blockv3,
            entity_data: Some(GameEntityData::Blockv3 {
                data: DespawnComponent {
                    camera_circle: SPAWN_RADIUS * 1.5,
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
    camera_query: Query<&Transform, With<Camera>>,
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
    camera_query: Query<&Transform, With<Camera>>,
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
            )) && (!despawn_tracker.spawned_id.contains(&building.id))
        });

    let to_loop = to_spawn.collect::<Vec<_>>();

    if to_loop.len() == 0 {
        return;
    }

    for forest in &to_loop {
        despawn_tracker.spawned_id.push(forest.id.clone());
        let mountain_1_points = forest
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
    // println!("---------------- Spawned: {}", to_loop.len());
}

fn boracay_road_stream_system(
    camera_query: Query<&Transform, With<Camera>>,
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
                    camera_circle: SPAWN_RADIUS * 1.5,
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

    commands
        .spawn(GeometryBuilder::build_as(
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
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(RigidBody::Fixed)
        .insert(Velocity::zero())
        .insert(Collider::polyline(mountain_1_points, Some(index_list)));
    // .insert(Collider::convex_decomposition(
    //     mountain_1_points.as_slice(),
    //     index_list.as_slice(),
    // ));
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
