// To describe how the Scene03 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{
        Collider, NoUserData, RapierConfiguration, RapierPhysicsPlugin, RigidBody, Velocity,
    },
    render::RapierDebugRenderPlugin,
};

use crate::{
    entity_factory::factory::data::{GameEntity, SpawnEntityEvent},
    game_modules::{
        controllable::components::ControllableComponent,
        save_load::{data::GlobalSaveData, systems::TriggerSaveLoadEvevnt},
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
            SystemSet::on_exit(GameScene::Scene03).with_system(despawn_screen::<World01>),
        );
    }
}

fn scene03_init_system(mut commands: Commands) {
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

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.0,
    });

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
    controllable_query: Query<&ControllableComponent>,
) {
    for controllable_component in controllable_query.iter() {
        if (!controllable_component.enabled) {
            continue;
        }

        if (controllable_component.btn_a.pressed) {
            spawn_entity_events.send(SpawnEntityEvent {
                entity: GameEntity::PlayerV2,
                ..Default::default()
            });
        }
    }
}
