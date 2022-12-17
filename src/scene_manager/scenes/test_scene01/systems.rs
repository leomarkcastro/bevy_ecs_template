// To describe how the Scene01 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::{
    entity_factory::factory::data::{GameEntity, SpawnEntityEvent},
    game_modules::{
        controllable::components::ControllableComponent, timers::components::OneSecondTimer,
    },
    scene_manager::manager::{
        entities::{SpawnAt, World01},
        scene_list::GameScene,
        utils::despawn_screen,
    },
};

pub struct Scene01Plugin;

impl Plugin for Scene01Plugin {
    fn build(&self, app: &mut App) {
        app // When entering the state, spawn everything needed for this screen
            .add_system_set(
                SystemSet::on_enter(GameScene::Scene01).with_system(scene01_init_system),
            )
            .add_system_set(
                SystemSet::on_update(GameScene::Scene01).with_system(scene01_timerspawning_system),
            )
            .add_system_set(
                SystemSet::on_update(GameScene::Scene01).with_system(scene01_clickspawning_system),
            )
            .add_system_set(
                SystemSet::on_update(GameScene::Scene01).with_system(scene01_camera_system),
            )
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(
                SystemSet::on_exit(GameScene::Scene01).with_system(despawn_screen::<World01>),
            );
    }
}

fn scene01_init_system() {
    println!("Scene01 init")
}

fn scene01_clickspawning_system(
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    controllable_query: Query<&ControllableComponent>,
) {
    for controllable_component in controllable_query.iter() {
        if (!controllable_component.enabled) {
            continue;
        }

        if (controllable_component.btn_a.pressed) {
            spawn_entity_events.send(SpawnEntityEvent {
                entity: GameEntity::PlayerV1,
                ..Default::default()
            });
        }
    }
}

fn scene01_timerspawning_system(
    time: Res<Time>,
    mut one_sec_timer: ResMut<OneSecondTimer>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
) {
    if one_sec_timer.event_timer.tick(time.delta()).finished() {
        // get camera x position
        let camera_x = camera_query.single_mut().translation.x;
        // generate random position
        let position = Vec3::new(
            camera_x + (rand::random::<f32>() * 600.0) - 300.0,
            (rand::random::<f32>() * 400.0) - 200.0,
            0.0,
        );
        spawn_entity_events.send(SpawnEntityEvent {
            position: Some(position),
            ..Default::default()
        });
    }

    // println!(".Scene01")
}

const CAMERA_SPEED: f32 = 1.0;

fn scene01_camera_system(time: Res<Time>, mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x += CAMERA_SPEED;
    // camera_transform.scale = Vec3::new(0.5, 0.5, 0.5);
}
