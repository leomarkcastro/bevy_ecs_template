// To describe how the Scene01 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::{
    entity_factory::factory::data::{GameEntity, SpawnEntityEvent},
    game_modules::{
        controllable::components::ControllableResource,
        save_load::{data::GlobalSaveData, systems::TriggerSaveLoadEvevnt},
        timers::components::OneSecondTimer,
    },
    scene_manager::manager::{
        entities::{SpawnAt, World01},
        scene_list::GameScene,
        utils::despawn_screen,
    },
};

pub struct Scene02Plugin;

impl Plugin for Scene02Plugin {
    fn build(&self, app: &mut App) {
        app // When entering the state, spawn everything needed for this screen
            .add_system_set(
                SystemSet::on_enter(GameScene::Scene02).with_system(scene02_init_system),
            )
            .add_system_set(
                SystemSet::on_update(GameScene::Scene02).with_system(scene02_clickspawning_system),
            )
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(
                SystemSet::on_exit(GameScene::Scene02).with_system(despawn_screen::<World01>),
            );
    }
}

fn scene02_init_system() {
    println!("Scene02 init")
}

fn scene02_clickspawning_system(
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    mut save_load_events: EventWriter<TriggerSaveLoadEvevnt>,
    controllable_component: Res<ControllableResource>,
) {
    if (!controllable_component.enabled) {
        return;
    }

    if (controllable_component.btn_a.pressed) {
        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::PlayerV1,
            ..Default::default()
        });
    }
    if (controllable_component.btn_b.pressed) {
        // generate random number
        let random_number = rand::random::<i32>();

        save_load_events.send(TriggerSaveLoadEvevnt {
            save: true,
            load: false,
            data: Some(GlobalSaveData {
                player_health: random_number,
            }),
        });
    }
    if (controllable_component.btn_c.pressed) {
        save_load_events.send(TriggerSaveLoadEvevnt {
            save: false,
            load: true,
            data: None,
        });
    }
}
