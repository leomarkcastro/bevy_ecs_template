// To describe how the Scene05 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    entity_factory::{
        entities::{playerv2::entities::Playerv2Entity, playerv3::entities::Playerv3Entity},
        factory::data::{GameEntity, SpawnEntityEvent},
    },
    game_modules::{
        camera::components::{CameraFollowable, CameraMode, CameraResource},
        controllable::components::ControllableResource,
        shaders::simple_point_light::components::CoolMaterialUniformInput,
    },
    scene_manager::manager::{entities::World01, scene_list::GameScene, utils::despawn_screen},
};

pub struct Scene05Plugin;

impl Plugin for Scene05Plugin {
    fn build(&self, app: &mut App) {
        app // When entering the state, spawn everything needed for this screen
            .add_system_set(
                SystemSet::on_enter(GameScene::Scene05).with_system(scene05_init_system),
            )
            .add_system_set(SystemSet::on_update(GameScene::Scene05).with_system(scene05_update))
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

fn scene05_init_system(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
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
