#![allow(unused)]
#![deny(clippy::all)]

mod entity_factory;
mod game_modules;
mod scene_manager;

use bevy::prelude::*;
use entity_factory::factory::systems::EntityFactoryPlugin;
use game_modules::{
    controllable::systems::ControllablePlugin, save_load::systems::SaveLoadFactoryPlugin,
    timers::systems::TimersPlugin,
};
use scene_manager::manager::systems::SceneManagerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Template Bevy Project".to_string(),
                        width: 640.0,
                        height: 480.0,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(AssetPlugin {
                    // This tells the AssetServer to watch for changes to assets.
                    // It enables our scenes to automatically reload in game when we modify their files.
                    // watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(SaveLoadFactoryPlugin)
        .add_plugin(TimersPlugin)
        .add_plugin(ControllablePlugin)
        .add_plugin(EntityFactoryPlugin)
        .add_plugin(SceneManagerPlugin)
        .run();
}
