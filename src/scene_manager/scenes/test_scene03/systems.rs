// To describe how the Scene03 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{
    prelude::{shape::Quad, *},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::{
    prelude::{
        Collider, NoUserData, RapierConfiguration, RapierPhysicsPlugin, RigidBody, Velocity,
    },
    render::RapierDebugRenderPlugin,
};

use crate::{
    entity_factory::{
        entities::playerv2::entities::Playerv2Entity,
        factory::data::{GameEntity, SpawnEntityEvent},
    },
    game_modules::{
        camera::components::{CameraFollowable, CameraMode, CameraResource},
        controllable::components::ControllableResource,
        save_load::{data::GlobalSaveData, systems::TriggerSaveLoadEvevnt},
        shaders::simple_point_light::components::{CoolMaterial, CoolMaterialUniformInput},
        timers::components::OneSecondTimer,
    },
    scene_manager::manager::{
        entities::{SpawnAt, World01},
        scene_list::GameScene,
        utils::despawn_screen,
    },
};

pub struct Scene03Plugin;

impl Plugin for Scene03Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameScene::Scene03).with_system(scene03_init_system),
        )
        .add_system_set(SystemSet::on_update(GameScene::Scene03).with_system(scene03_run_system))
        .add_system_set(
            SystemSet::on_update(GameScene::Scene03)
                .with_system(scene03_follow_first_player_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameScene::Scene03).with_system(despawn_screen::<World01>),
        );
    }
}

fn scene03_init_system(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<CoolMaterial>>,
    mut camera_resource: ResMut<CameraResource>,
) {
    // camera_resource.mode = CameraMode::AtPoint {
    //     target_point: Vec3::new(60.0, 0.0, 0.0),
    // };
    println!("Scene03 init");
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

fn scene03_run_system(
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    controllable_component: Res<ControllableResource>,
) {
    if (!controllable_component.enabled) {
        return;
    }

    if (controllable_component.btn_a.pressed) {
        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::PlayerV2,
            ..Default::default()
        });
    }
}

fn scene03_follow_first_player_system(
    mut camera_resource: ResMut<CameraResource>,
    player_query: Query<(&CameraFollowable), With<Playerv2Entity>>,
) {
    if let Ok((&followable)) = player_query.get_single() {
        if let CameraMode::AtAssetFace {
            target_asset_id,
            distance,
        } = camera_resource.mode
        {
            return;
        }
        // println!("CameraMode::AtAsset");
        let followable_id = followable.id;
        camera_resource.mode = CameraMode::AtAssetFace {
            target_asset_id: followable_id,
            distance: 30.0,
        };
        camera_resource.speed = 0.04;
    }
}
