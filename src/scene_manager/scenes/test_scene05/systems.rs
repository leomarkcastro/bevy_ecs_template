// To describe how the Scene05 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;
use bevy::{math::Vec3Swizzles, utils::HashSet};
use bevy_ecs_tilemap::prelude::TilemapRenderSettings;
use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize,
        TilemapType,
    },
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, RigidBody, Velocity};
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    entity_factory::{
        entities::{playerv2::entities::Playerv2Entity, playerv3::entities::Playerv3Entity},
        factory::data::{GameEntity, SpawnEntityEvent},
    },
    game_modules::{
        camera::components::{CameraFollowable, CameraMode, CameraResource},
        controllable::components::ControllableResource,
        map_loader::systems::{MapDataResource, TileDataResource},
        shaders::simple_point_light::components::CoolMaterialUniformInput,
    },
    scene_manager::manager::{entities::World01, scene_list::GameScene, utils::despawn_screen},
    utils::globals::MAP_SCALE,
};

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
    x: CHUNK_SIZE.x * 16,
    y: CHUNK_SIZE.y * 16,
};
const TRIGGER_SPAWN_RADIUS: f32 = 50.0 * MAP_SCALE;
const SPAWN_RADIUS: f32 = TRIGGER_SPAWN_RADIUS * 2. * MAP_SCALE;

// Shit ball park value for now
const TILE_INDEX_PADDING: IVec2 = IVec2::new(18 - 5, 292 - 68);

#[derive(Resource, Default)]
struct IslandTileMapGlobals {
    island_tile_cam_pos: Option<Vec2>,
    mountain_tile_cam_pos: Option<Vec2>,
}

pub struct Scene05Plugin;

impl Plugin for Scene05Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
        })
        .insert_resource(IslandTileMapGlobals::default())
        .insert_resource(ChunkManager::default()) // When entering the state, spawn everything needed for this screen
        .add_system_set(SystemSet::on_enter(GameScene::Scene05).with_system(scene05_init_system))
        .add_system_set(SystemSet::on_update(GameScene::Scene05).with_system(scene05_update))
        .add_system_set(SystemSet::on_update(GameScene::Scene05).with_system(stream_island_tiles))
        .add_system_set(SystemSet::on_update(GameScene::Scene05).with_system(stream_mountain_tiles))
        .add_system_set(SystemSet::on_update(GameScene::Scene05).with_system(despawn_tiles))
        .add_system_set(
            SystemSet::on_update(GameScene::Scene05)
                .with_system(scene05_follow_first_player_system),
        )
        // When exiting the state, despawn everything that was spawned for this screen
        .add_system_set(
            SystemSet::on_exit(GameScene::Scene05).with_system(despawn_screen::<World01>),
        );
    }
}

fn load_island_tile(
    mut commands: &mut Commands,
    tile_data_resource: &Res<TileDataResource>,
    asset_server: &Res<AssetServer>,
) {
    // let image: Handle<Image> = asset_server.load("image_tileset/grass/07.png");
    // scale the image
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

    let tile_data = tile_data_resource.island_tile_data.as_ref().unwrap();

    let map_size = TilemapSize {
        x: tile_data.xsize,
        y: tile_data.ysize,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let mut processed_tiles = 0;
    println!("tile_data [{}, {}]", tile_data.xsize, tile_data.ysize);

    for x_index in 0..map_size.x {
        for y_index in 0..map_size.y {
            // if let Some(tile_cell_data) = tile_data
            //     .points
            //     .get(&format!("{:04}_{:04}", x_index, y_index))
            // {
            let tile = tile_data
                .points
                .get(&format!("{:04}_{:04}", x_index, y_index));
            if tile.is_none() {
                continue;
            }
            let tile = match tile {
                Some(tile_dat) => tile_dat.tile - 1,
                None => transparent_index,
            };
            let tile_pos = TilePos {
                x: x_index,
                y: y_index,
            };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
            processed_tiles += 1;
            // };
        }
    }

    println!("processed_tiles: {}", processed_tiles);

    let tile_size = TilemapTileSize::from(TILE_SIZE);
    let grid_size = bevy_ecs_tilemap::prelude::TilemapGridSize::from(TILE_SIZE);
    let map_type = TilemapType::default();

    let mut tilemap_bundle = TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: texture_vec,
        tile_size,
        // transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0),
        ..Default::default()
    };

    // compute tilemap width and height
    let tilemap_width = (tilemap_bundle.size.x as f32) * 0.15 * tilemap_bundle.grid_size.x;
    let tilemap_height = (tilemap_bundle.size.y as f32) * 0.9885 * tilemap_bundle.grid_size.y;
    println!("tilemap_width: {}", tilemap_width);
    println!("tilemap_height: {}", tilemap_height);

    // set tilemap center
    tilemap_bundle.transform.translation = Vec3 {
        x: -tilemap_width,
        y: -tilemap_height,
        z: 0.0,
    };

    // tilemap_bundle.transform.scale = Vec3::new(MAP_SCALE, MAP_SCALE, 1.0);

    commands.entity(tilemap_entity).insert((tilemap_bundle));
}

fn load_mntn_tile(
    mut commands: &mut Commands,
    tile_data_resource: &Res<TileDataResource>,
    asset_server: &Res<AssetServer>,
) {
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
    ];
    let transparent_index = image_handles.len() as u32 - 1;
    let texture_vec = TilemapTexture::Vector(image_handles);

    let tile_data = tile_data_resource.mountain_tile_data.as_ref().unwrap();

    let map_size = TilemapSize {
        x: tile_data.xsize,
        y: tile_data.ysize,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let mut processed_tiles = 0;
    println!("tile_data [{}, {}]", tile_data.xsize, tile_data.ysize);

    for x_index in 0..map_size.x {
        for y_index in 0..map_size.y {
            // if let Some(tile_cell_data) = tile_data
            //     .points
            //     .get(&format!("{:04}_{:04}", x_index, y_index))
            // {
            let tile = tile_data
                .points
                .get(&format!("{:04}_{:04}", x_index, y_index));
            if tile.is_none() {
                continue;
            }
            let tile = match tile {
                Some(tile_dat) => tile_dat.tile - 1,
                None => transparent_index,
            };
            let tile_pos = TilePos {
                x: x_index,
                y: y_index,
            };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
            processed_tiles += 1;
            // };
        }
    }

    println!("processed_tiles: {}", processed_tiles);

    let tile_size = TilemapTileSize::from(TILE_SIZE);
    let grid_size = bevy_ecs_tilemap::prelude::TilemapGridSize::from(TILE_SIZE);
    let map_type = TilemapType::default();

    let mut tilemap_bundle = TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: texture_vec,
        tile_size,
        // transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 3.0),
        ..Default::default()
    };
    // compute tilemap width and height
    let tilemap_width = (tilemap_bundle.size.x as f32) * 0.15 * tilemap_bundle.grid_size.x;
    let tilemap_height = (tilemap_bundle.size.y as f32) * 0.9885 * tilemap_bundle.grid_size.y;
    // println!("tilemap_width: {}", tilemap_width);
    // println!("tilemap_height: {}", tilemap_height);

    // set tilemap center
    tilemap_bundle.transform.translation = Vec3 {
        x: -tilemap_width,
        y: -tilemap_height,
        z: 0.0,
    };

    commands.entity(tilemap_entity).insert((tilemap_bundle));
}

fn add_collission(
    mut commands: &mut Commands,
    map_data: &Res<MapDataResource>,
    mut spawn_entity_events: &mut EventWriter<SpawnEntityEvent>,
) {
    let map_data = map_data.map_data.as_ref().unwrap();

    let island = &map_data.land_vectorpoints_outline;

    let mountain_1_points = island
        .points_less
        .clone()
        .into_iter()
        .map(|p| Vec2::new(p.x, -p.y) * MAP_SCALE)
        .collect::<Vec<Vec2>>();

    // crete [0,1,2 ... n] index for mountain_1_points
    let mut index = 0;
    let mut index_list = Vec::new();
    for _ in mountain_1_points.iter().skip(1) {
        index_list.push([index, index + 1]);
        index += 1;
    }

    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: island.start.extend(0.0)
                    * Vec3 {
                        x: 1.0,
                        y: -1.0,
                        z: 1.0,
                    }
                    * MAP_SCALE,
                ..Default::default()
            },
            ..Default::default()
        })
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
    println!("Streaming island");
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
        0.0,
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
    mut scene_global: ResMut<IslandTileMapGlobals>,
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
    println!("Camera moved");

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

fn spawn_mountain_chunk(
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
    ];
    let transparent_index = image_handles.len() as u32 - 1;
    let texture_vec = TilemapTexture::Vector(image_handles);

    // Load tile data
    let tile_data = tile_data_resource.mountain_tile_data.as_ref().unwrap();
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
        ((chunk_pos.x - TILE_INDEX_PADDING.x) as f32 - RANGE as f32 - 0.45) * TILE_SIZE.x,
        ((chunk_pos.y - TILE_INDEX_PADDING.y) as f32 - RANGE as f32 + 0.55) * TILE_SIZE.y,
        2.0,
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
    mut scene_global: ResMut<IslandTileMapGlobals>,
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
    println!("Camera moved");

    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        let camera_chunk_pos_idd = IVec3::new(camera_chunk_pos.x, camera_chunk_pos.y, 1);
        // println!("chunk: {:?}", camera_chunk_pos);
        if !chunk_manager.spawned_chunks.contains(&camera_chunk_pos_idd) {
            chunk_manager.spawned_chunks.insert(camera_chunk_pos_idd);
            spawn_mountain_chunk(
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
            if distance > SPAWN_RADIUS * 2.0 {
                let x = (chunk_pos.x as f32 / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y as f32 / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec3::new(x, y, 0));
                chunk_manager.spawned_chunks.remove(&IVec3::new(x, y, 1));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn scene05_init_system(
    mut commands: Commands,
    // tiles
    tile_data_resource: Res<TileDataResource>,
    asset_server: Res<AssetServer>,
    // collission
    map_data: Res<MapDataResource>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
) {
    // load_island_tile(&mut commands, &tile_data_resource, &asset_server);
    // load_mntn_tile(&mut commands, &tile_data_resource, &asset_server);
    add_collission(&mut commands, &map_data, &mut spawn_entity_events);
    println!("scene05_init_system");
}

fn scene05_update(
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    controllable_component: Res<ControllableResource>,
) {
    if (controllable_component.btn_c.pressed) {
        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::PlayerV3,
            position: Some(Vec3::from([0.0, 0.0, 20.0])),
            ..Default::default()
        });
    }
}

fn scene05_follow_first_player_system(
    mut camera_resource: ResMut<CameraResource>,
    mut colordata_query: Query<
        (&mut CoolMaterialUniformInput, &Transform),
        Without<Playerv3Entity>,
    >,
    player_query: Query<(&CameraFollowable, &mut Transform), With<Playerv3Entity>>,
) {
    if let Ok((&pl_followable, mut pl_transform)) = player_query.get_single() {
        if let CameraMode::AtAssetFace {
            target_asset_id,
            distance,
        } = camera_resource.mode
        {
            return;
        }
        // println!("CameraMode::AtAsset");
        let followable_id = pl_followable.id;
        camera_resource.mode = CameraMode::AtAssetFace {
            target_asset_id: followable_id,
            distance: 30.0,
        };
        camera_resource.speed = 0.04;
    }
}
