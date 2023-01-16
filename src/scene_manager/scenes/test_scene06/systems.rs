// To describe how the Scene06 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{
    prelude::{shape::Quad, *},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
    utils::Uuid,
};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder, PathBuilder, ShapePlugin, StrokeMode},
    shapes,
};
use bevy_rapier2d::{
    prelude::{
        Collider, NoUserData, RapierConfiguration, RapierPhysicsPlugin, RigidBody, Velocity,
    },
    render::RapierDebugRenderPlugin,
};

use crate::{
    entity_factory::{
        entities::{
            global::{
                despawn::components::DespawnComponent,
                proximity::components::ProximityDataComponent,
            },
            pickupablev1::entities::Pickupablev1Entity,
            playerv2::entities::Playerv2Entity,
            playerv3::entities::Playerv3Entity,
        },
        factory::data::{GameEntity, GameEntityData, SpawnEntityEvent, SpawnUIEvent},
    },
    game_modules::{
        camera::components::{CameraFollowable, CameraMode, CameraResource},
        controllable::components::ControllableResource,
        global_event::systems::GlobalEvent,
        map_loader::{
            boracay::{embed_boracy_map, BoracayMapPlugin},
            boracayv2::embed_boracyv2_map,
            data::RoomType,
            systems::MapDataResource,
        },
        save_load::{data::GlobalSaveData, systems::TriggerSaveLoadEvevnt},
        shaders::simple_point_light::components::{CoolMaterial, CoolMaterialUniformInput},
        time_system::systems::CurrentWorldTimeGlobal,
        timers::components::OneSecondTimer,
    },
    scene_manager::manager::{
        entities::{SpawnAt, World01},
        scene_list::GameScene,
        utils::despawn_screen,
    },
};

#[derive(Resource)]
struct Scene06Globals {
    current_pointer: Option<Uuid>,
    wall_counter: u32,
}

impl Default for Scene06Globals {
    fn default() -> Self {
        Self {
            current_pointer: None,
            wall_counter: 0,
        }
    }
}

pub struct Scene06Plugin;

impl Plugin for Scene06Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scene06Globals::default());

        embed_boracyv2_map(
            app,
            Some((
                SystemSet::on_enter(GameScene::Scene06),
                SystemSet::on_update(GameScene::Scene06),
                SystemSet::on_exit(GameScene::Scene06),
            )),
        );

        app.add_system_set(
            SystemSet::on_enter(GameScene::Scene06).with_system(scene06_init_system),
        )
        .add_system_set(SystemSet::on_update(GameScene::Scene06).with_system(scene06_run_system))
        .add_system_set(
            SystemSet::on_update(GameScene::Scene06).with_system(scene06_progression_system),
        )
        // .add_system_set(
        //     SystemSet::on_update(GameScene::Scene06).with_system(scene06_timerspawning_system),
        // )
        .add_system_set(
            SystemSet::on_update(GameScene::Scene06)
                .with_system(scene06_follow_first_player_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameScene::Scene06).with_system(despawn_screen::<World01>),
        );
    }
}

fn scene06_init_system(
    mut commands: Commands,
    mut time_tick: ResMut<CurrentWorldTimeGlobal>,
    // mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    // mut mesh_assets: ResMut<Assets<Mesh>>,
    // mut my_material_assets: ResMut<Assets<CoolMaterial>>,
    // mut camera_resource: ResMut<CameraResource>,
    // map_data: Res<MapDataResource>,
) {
    // camera_resource.mode = CameraMode::AtPoint {
    //     target_point: Vec3::new(60.0, 0.0, 0.0),
    // };
    println!("Scene06 init");

    time_tick.active = true;

    // let map_data = map_data.map_data.as_ref().unwrap();

    // for building in &map_data.buildings {
    //     if (building.bldg_type == "mountain" || building.bldg_type == "forest") {
    //         continue;
    //     }
    //     spawn_entity_events.send(SpawnEntityEvent {
    //         entity: GameEntity::Blockv1,
    //         position: Some(
    //             (building.center.extend(0.0) / 10.0)
    //                 * Vec3 {
    //                     x: 1.0,
    //                     y: -1.0,
    //                     z: 1.0,
    //                 },
    //         ),
    //         size: Some(Vec2::new(building.width, building.height) / 10.0),
    //         ..Default::default()
    //     });
    // }

    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgba(0.0, 0.0, 0.0, 0.995),
    //         custom_size: Some(Vec2::new(1000.0, 1000.0)),
    //         ..Default::default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 0.0, 3.0),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });

    // commands
    //     .spawn(MaterialMesh2dBundle {
    //         mesh: mesh_assets
    //             .add(Mesh::from(shape::Quad::from(Quad {
    //                 size: Vec2::new(50., 50.),
    //                 ..Default::default()
    //             })))
    //             .into(),
    //         material: my_material_assets.add(CoolMaterial {
    //             ..Default::default()
    //         }),
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         ..default()
    //     })
    //     .insert(CoolMaterialUniformInput {
    //         color: Color::rgba(0.0, 0.0, 0.0, 0.75),
    //         ..Default::default()
    //     });

    // light
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: false,
    //         color: Color::WHITE,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });
}

const SCENE_ID: &str = "test-scene06";

fn scene06_run_system(
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    mut spawn_ui_events: EventWriter<SpawnUIEvent>,
    mut scene_global: ResMut<Scene06Globals>,
    controllable_component: Res<ControllableResource>,
) {
    if (!controllable_component.enabled) {
        return;
    }

    // if (controllable_component.btn_b.pressed) {
    //     let pointer = SpawnEntityEvent {
    //         entity: GameEntity::Pickupablev1,
    //         entity_data: Some(GameEntityData::Pickupablev1 {
    //             on_pickup: GlobalEvent {
    //                 event_data: "Pointer".to_string(),
    //                 scene_id: SCENE_ID.to_string(),
    //             },
    //         }),
    //         ..Default::default()
    //     };
    //     let pointer_id = pointer.id.clone();
    //     scene_global.current_pointer = Some(pointer_id);
    //     spawn_entity_events.send(pointer);
    // }

    // if (controllable_component.btn_b.pressed) {
    //     // let pointer_id = pointer.id.clone();
    //     // scene_global.current_pointer = Some(pointer_id);
    //     spawn_entity_events.send(SpawnEntityEvent {
    //         entity: GameEntity::Blockv1,
    //         position: Some(Vec3::from([
    //             0.0,
    //             scene_global.wall_counter as f32 * 10.0,
    //             0.0,
    //         ])),
    //         ..Default::default()
    //     });
    //     spawn_entity_events.send(SpawnEntityEvent {
    //         entity: GameEntity::Blockv2,
    //         position: Some(Vec3::from([
    //             20.0,
    //             scene_global.wall_counter as f32 * 10.0,
    //             0.0,
    //         ])),
    //         ..Default::default()
    //     });
    //     scene_global.wall_counter += 1;
    // }

    // if (controllable_component.btn_b.pressed) {
    //     spawn_entity_events.send(SpawnEntityEvent {
    //         entity: GameEntity::Roomv1,
    //         entity_data: Some(GameEntityData::Roomv1 {
    //             room_type: RoomType::House,
    //             despawn_data: DespawnComponent {
    //                 ..Default::default()
    //             },
    //         }),
    //         position: Some(Vec3::from([0.0, 0.0, 0.0])),
    //         size: Some(Vec2::from([100.0, 100.0])),
    //         ..Default::default()
    //     });
    // }

    // if (controllable_component.btn_b.pressed) {
    //     spawn_entity_events.send(SpawnEntityEvent {
    //         entity: GameEntity::Treev2,
    //         entity_data: Some(GameEntityData::Treev1 {
    //             despawn_data: DespawnComponent {
    //                 ..Default::default()
    //             },
    //             internal_radius_percentage: 0.12,
    //         }),
    //         position: Some(Vec3::from([0.0, 0.0, 200.0])),
    //         size: Some(Vec2::from([100.0, 100.0])),
    //         ..Default::default()
    //     });
    // }

    if (controllable_component.btn_c.pressed) {
        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::PlayerV3,
            position: Some(Vec3::from([0.0, 0.0, 130.0])),
            ..Default::default()
        });
    }

    // if (controllable_component.btn_d.pressed) {
    //     spawn_entity_events.send(SpawnEntityEvent {
    //         entity: GameEntity::Zombiesv2,
    //         ..Default::default()
    //     });
    // }
}

fn scene06_timerspawning_system(
    time: Res<Time>,
    mut one_sec_timer: ResMut<OneSecondTimer>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    mut scene_global: ResMut<Scene06Globals>,
) {
    if (scene_global.wall_counter > 20) {
        return;
    }
    if one_sec_timer.event_timer.tick(time.delta()).finished() {
        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::Blockv1,
            position: Some(Vec3::from([
                0.0,
                scene_global.wall_counter as f32 * 10.0,
                0.0,
            ])),
            size: Some(Vec2::from([50.0, 10.0])),
            ..Default::default()
        });
        // spawn_entity_events.send(SpawnEntityEvent {
        //     entity: GameEntity::Blockv2,
        //     position: Some(Vec3::from([
        //         20.0,
        //         scene_global.wall_counter as f32 * 10.0,
        //         0.0,
        //     ])),
        //     ..Default::default()
        // });
        scene_global.wall_counter += 1;
    }

    // println!(".Scene01")
}

fn scene06_progression_system(
    mut command: Commands,
    mut pickable_query: Query<(Entity, &ProximityDataComponent), With<Pickupablev1Entity>>,
    mut global_even_read: EventReader<GlobalEvent>,
    scene_global: Res<Scene06Globals>,
) {
    let scene_events = global_even_read.iter().filter(|e| e.scene_id == SCENE_ID);
    for event in scene_events {
        match event.event_data.as_str() {
            "Pointer" => {
                let mut query = pickable_query.iter().filter(|(_, proximity_data)| {
                    proximity_data.id == scene_global.current_pointer.unwrap_or_default()
                });

                if let Some((entity, _)) = query.next() {
                    command.entity(entity).despawn();
                }
            }
            _ => {}
        }
    }
}

fn scene06_follow_first_player_system(
    mut camera_resource: ResMut<CameraResource>,
    mut colordata_query: Query<
        (&mut CoolMaterialUniformInput, &Transform),
        Without<Playerv3Entity>,
    >,
    player_query: Query<(&CameraFollowable, &mut Transform), With<Playerv3Entity>>,
) {
    if let Ok((&pl_followable, mut pl_transform)) = player_query.get_single() {
        if let (Ok((mut cd_colordata, cd_transform))) = colordata_query.get_single_mut() {
            cd_colordata.position[0].x = pl_transform.translation.x;
            cd_colordata.position[0].y = pl_transform.translation.y;
            cd_colordata.position[0].z = 1000.0;
        }
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
