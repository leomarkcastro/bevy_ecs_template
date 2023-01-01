// To describe how the SimpleText component/entity should behave.
// WILL: contain pure logic that interacts with the component

use std::fs::read_to_string;

use bevy::{
    prelude::*,
    tasks::{IoTaskPool, Task},
};
use bevy_rapier2d::na::Point;
use knn::PointCloud;
use serde_json::Map;

use crate::game_modules::timers::components::{OneSecondTimer, ThreeSecondTimer};

use super::data::{MapData, RoomData};

#[derive(Resource)]
pub struct MapDataResource {
    pub map_data: Option<MapData>,
}

#[derive(Resource)]
pub struct RoomDataResource {
    pub room_data: Option<RoomData>,
}

pub struct MapLoaderPlugin;

impl Plugin for MapLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapDataResource { map_data: None })
            .insert_resource(RoomDataResource { room_data: None })
            .add_startup_system(map_loader_system)
            .add_startup_system(room_loader_system);
        // .add_system(map_knn_test_system);
    }
}

pub fn map_loader_system(mut map_data: ResMut<MapDataResource>) {
    // load map data json from asset server

    let data =
        read_to_string(format!("assets/maps/boracay_map.json")).expect("Unable to load map file");

    let loaded_data: Result<MapData, _> = serde_json::from_str(&data);

    match loaded_data {
        Ok(loaded_data) => {
            map_data.map_data = Some(loaded_data);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // map_data.map_data = Some(loaded_data.unwrap());
}

pub fn room_loader_system(mut room_data: ResMut<RoomDataResource>) {
    // load map data json from asset server

    let data = read_to_string(format!("assets/maps/boracay_rooms.json"))
        .expect("Unable to load room file");

    let loaded_data: Result<RoomData, _> = serde_json::from_str(&data);

    match loaded_data {
        Ok(loaded_data) => {
            room_data.room_data = Some(loaded_data);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn map_knearest(
    map_data: &Res<MapDataResource>,
    point: &Vec2,
    k: usize,
) -> Vec<(f64, (f64, f64, String))> {
    let distance_function =
        |p: &(f64, f64, &str), q: &(f64, f64, &str)| (q.0 - p.0).abs() + (q.1 - p.1).abs();
    let mut pc = PointCloud::new(distance_function);
    let md = map_data.map_data.as_ref().unwrap();
    let center_points: Vec<(f64, f64, &str)> = md
        .buildings
        .iter()
        .map(|building| {
            let center = building.center;
            (center.x as f64, center.y as f64, building.id.as_str())
        })
        .collect();
    for i in 0..center_points.len() {
        pc.add_point(&center_points[i]);
    }

    let points = pc.get_nearest_k(&(point.x as f64, point.y as f64, ""), k);
    let mut result: Vec<(f64, (f64, f64, String))> = Vec::new();
    for i in 0..points.len() {
        let p = points[i];
        result.push((p.0, (p.1 .0, p.1 .1, p.1 .2.to_string())));
    }

    result
}

fn map_knn_test_system(
    map_data: Res<MapDataResource>,
    time: Res<Time>,
    mut one_sec_timer: ResMut<ThreeSecondTimer>,
) {
    if one_sec_timer.event_timer.tick(time.delta()).finished() {
        println!(
            "KNN test: {:?}",
            map_knearest(&map_data, &Vec2 { x: 6., y: 6. }, 30)
        );
    }
}
