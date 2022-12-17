// To describe how the __templateNameToPascalCase__ component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::scene_manager::manager::{scene_list::GameScene, utils::despawn_screen};

// Tag component used to tag entities added on this screen
#[derive(Component)]
struct On__templateNameToPascalCase__;

pub struct __templateNameToPascalCase__Plugin;

impl Plugin for __templateNameToPascalCase__Plugin {
    fn build(&self, app: &mut App) {
        app // When entering the state, spawn everything needed for this screen
            .add_system_set(
                SystemSet::on_enter(GameScene::__templateNameToPascalCase__)
                    .with_system(__templateNameToLowerCase___init_system),
            )
            // While in this state, run the `countdown` system
            .add_system_set(
                SystemSet::on_update(GameScene::__templateNameToPascalCase__)
                    .with_system(__templateNameToLowerCase___system),
            )
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(
                SystemSet::on_exit(GameScene::__templateNameToPascalCase__)
                    .with_system(despawn_screen::<On__templateNameToPascalCase__>),
            );
    }
}

fn __templateNameToLowerCase___init_system() {
    println!("__templateNameToPascalCase__ init")
}

fn __templateNameToLowerCase___system() {
    println!(".__templateNameToPascalCase__")
}
