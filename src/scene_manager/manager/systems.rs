// To describe how the Manager component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::{
    game_modules::controllable::{components::ControllableComponent, entities::ControllableEntity},
    scene_manager::scenes::{inject_scenes, test_scene01::systems::Scene01Plugin, DEFAULT_SCENE},
};

use super::scene_list::GameScene;

pub struct ChangeSceneEvent {
    pub to_scene: GameScene,
}

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Declare the game state, and set its startup value
            .add_state(DEFAULT_SCENE)
            .add_event::<ChangeSceneEvent>()
            .add_startup_system(manager_init_system)
            .add_system(manager_system);
        // .add_system(manager_test_system)

        inject_scenes(app);
    }
}

fn manager_init_system(mut commands: Commands) {
    // IDK if i should put it here or on their respective scenes
    commands.spawn(Camera2dBundle::default());
}

fn manager_system(
    mut game_scene: ResMut<State<GameScene>>,
    mut change_scene_event_reader: EventReader<ChangeSceneEvent>,
) {
    for event in change_scene_event_reader.iter() {
        if game_scene.current() == &event.to_scene {
            continue;
        }
        println!("Changing scene to: {:?}", event.to_scene);
        game_scene.set(event.to_scene.clone()).unwrap();
    }
}

fn manager_test_system(
    control_query: Query<&ControllableComponent, With<ControllableEntity>>,
    mut change_scene_event_writer: EventWriter<ChangeSceneEvent>,
) {
    for mut controllable_component in control_query.iter() {
        if (!controllable_component.enabled) {
            continue;
        }

        if (controllable_component.btn_a.pressed) {
            change_scene_event_writer.send(ChangeSceneEvent {
                to_scene: GameScene::Scene02,
            });
        }

        if (controllable_component.btn_b.pressed) {
            change_scene_event_writer.send(ChangeSceneEvent {
                to_scene: GameScene::Scene03,
            });
        }

        if (controllable_component.btn_c.pressed) {
            change_scene_event_writer.send(ChangeSceneEvent {
                to_scene: GameScene::Scene01,
            });
        }
    }
}
