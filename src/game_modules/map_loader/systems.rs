// To describe how the SimpleText component/entity should behave.
// WILL: contain pure logic that interacts with the component

use std::fs::read_to_string;

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_rapier2d::na::Point;
use futures_lite::future;
use knn::PointCloud;
use serde_json::Map;

use crate::game_modules::{
    dynamic_data::{
        animation_definition::AnimationDefinition, mapdata_definition::MapDataDefinition,
    },
    timers::components::{OneSecondTimer, ThreeSecondTimer},
};

use kdtree::distance::squared_euclidean;
use kdtree::ErrorKind;
use kdtree::KdTree;

use super::data::{MapData, PathData, RoomData, TileData};

#[derive(Resource)]
pub struct MapDataResource {
    pub map_data: Option<MapDataDefinition>,
}

#[derive(Resource)]
pub struct RoomDataResource {
    pub room_data: Option<RoomData>,
}

#[derive(Resource)]
pub struct PathDataResource {
    pub path_data: Option<PathData>,
    pub kdtree: Option<KdTree<f32, usize, [f32; 2]>>,
    pub kdtree_task: Option<Task<KdTree<f32, usize, [f32; 2]>>>,
}

#[derive(Resource)]
pub struct TileDataResource {
    pub island_tile_data: Option<TileData>,
    pub mountain_tile_data: Option<TileData>,
    pub cement_tile_data: Option<TileData>,
}

pub struct MapLoaderPlugin;

impl Plugin for MapLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapDataResource { map_data: None })
            .insert_resource(RoomDataResource { room_data: None })
            .insert_resource(PathDataResource {
                path_data: None,
                kdtree: None,
                kdtree_task: None,
            })
            .insert_resource(TileDataResource {
                island_tile_data: None,
                mountain_tile_data: None,
                cement_tile_data: None,
            })
            .add_startup_system(load_json_assets)
            .add_startup_system(map_loader_system.after(load_json_assets))
            .add_startup_system(path_loader_system.after(load_json_assets))
            .add_startup_system(tile_loader_system.after(load_json_assets))
            .add_startup_system(room_loader_system.after(load_json_assets))
            .add_system(async_resolution_system);
        // .add_system(map_knn_test_system);
    }
}

#[derive(Resource)]
pub struct LoadedMapDataDefinitionHandle(Handle<MapDataDefinition>);

pub fn load_json_assets(mut command: Commands, assets_server: Res<AssetServer>) {
    let def_data: Handle<MapDataDefinition> = assets_server.load("maps/boracay.map.json");
    println!("Loaded map data definition");
    command.insert_resource(LoadedMapDataDefinitionHandle(def_data));
}

pub fn map_loader_system(
    mut map_data: ResMut<MapDataResource>,
    data_def_handle: Res<LoadedMapDataDefinitionHandle>,
    mut data_def_collection: ResMut<Assets<MapDataDefinition>>,
) {
    println!("Loading map data | len: {}", data_def_collection.len());
    if let Some(loaded_data) = data_def_collection.get(&data_def_handle.0) {
        println!("Loaded map data");
        map_data.map_data = Some(loaded_data.to_owned());
    }

    /*

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

    */

    // map_data.map_data = Some(loaded_data.unwrap());
}

pub fn tile_loader_system(mut tile_data: ResMut<TileDataResource>) {
    // load map data json from asset server

    let data = read_to_string(format!("assets/maps/boracay_grass.tiledata.json"))
        .expect("Unable to load tile file");

    let loaded_data: Result<TileData, _> = serde_json::from_str(&data);

    match loaded_data {
        Ok(loaded_island_data) => {
            tile_data.island_tile_data = Some(loaded_island_data);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let mountain_data = read_to_string(format!("assets/maps/boracay_mountain.tiledata.json"))
        .expect("Unable to load tile file");

    let loaded_mntn_data: Result<TileData, _> = serde_json::from_str(&mountain_data);

    match loaded_mntn_data {
        Ok(loaded_mntn_data) => {
            tile_data.mountain_tile_data = Some(loaded_mntn_data);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let cement_data = read_to_string(format!("assets/maps/boracay_cement.tiledata.json"))
        .expect("Unable to load tile file");

    let loaded_cmnt_data: Result<TileData, _> = serde_json::from_str(&cement_data);

    match loaded_cmnt_data {
        Ok(loaded_cmnt_data) => {
            tile_data.cement_tile_data = Some(loaded_cmnt_data);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // map_data.map_data = Some(loaded_data.unwrap());
}

pub fn room_loader_system(mut room_data: ResMut<RoomDataResource>) {
    // load map data json from asset server

    let data = read_to_string(format!("assets/maps/boracay.rooms.json"))
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

pub fn path_loader_system(mut path_data: ResMut<PathDataResource>) {
    // load map data json from asset server

    let data =
        read_to_string(format!("assets/maps/boracay.path.json")).expect("Unable to load room file");

    let loaded_data: Result<PathData, _> = serde_json::from_str(&data);

    match loaded_data {
        Ok(loaded_data) => {
            println!(
                "Loaded path data [* {}] [-> {}]",
                loaded_data.points.len(),
                loaded_data.vertices.len()
            );
            path_data.path_data = Some(loaded_data);

            let points_clone = path_data.path_data.as_mut().unwrap().points.clone();

            let task = AsyncComputeTaskPool::get().spawn(async move {
                let mut kdtree = KdTree::new(2);

                for i in 0..points_clone.len() {
                    let point = points_clone[i];
                    kdtree.add([point.x, point.y], i).unwrap_or(());
                }
                println!("KdTree build done");
                kdtree
            });

            // path_data.kdtree_task = Some(task);

            // get the result of the task
            // path_data.kdtree = Some(task.get(0).unwrap().clone());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn async_resolution_system(mut path_data: ResMut<PathDataResource>) {
    let task = path_data.kdtree_task.as_mut();
    if task.is_some() {
        if let Some(data) = future::block_on(future::poll_once(task.unwrap())) {
            println!("KdTree loaded with {} points", data.size());
            path_data.kdtree_task = None;
            path_data.kdtree = Some(data);
        }
    }
}

fn map_knn_test_system(
    map_data: Res<MapDataResource>,
    time: Res<Time>,
    mut one_sec_timer: ResMut<ThreeSecondTimer>,
) {
    // if one_sec_timer.event_timer.tick(time.delta()).finished() {
    //     println!(
    //         "KNN test: {:?}",
    //         map_knearest(&map_data, &Vec2 { x: 6., y: 6. }, 30)
    //     );
    // }
}
