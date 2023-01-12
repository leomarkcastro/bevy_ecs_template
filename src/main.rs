#![allow(unused)]
// #![deny(clippy::all)]

mod entity_factory;
mod game_modules;
mod gui;
mod scene_manager;
mod utils;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use entity_factory::factory::systems::EntityFactoryPlugin;
use game_modules::{
    camera::systems::CameraSetupPlugin, controllable::systems::ControllablePlugin,
    face_axis::systems::FaceAxisPlugin, global_event::systems::GlobalEventPlugin,
    kayak::systems::KayakPlugin, load_assets::systems::LoadAssetsPlugin,
    map_loader::systems::MapLoaderPlugin, pan_camera::systems::PanCameraPlugin,
    path_finding::system::PathFindingServerPlugin, save_load::systems::SaveLoadFactoryPlugin,
    shaders::systems::ShadersPlugin, time_system::systems::TimeSystemPlugin,
    timers::systems::TimersPlugin,
};
use scene_manager::manager::systems::SceneManagerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Template Bevy Project".to_string(),
                        width: 800.0,
                        height: 600.0,
                        present_mode: PresentMode::AutoVsync,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(AssetPlugin {
                    // This tells the AssetServer to watch for changes to assets.
                    // It enables our scenes to automatically reload in game when we modify their files.
                    // watch_for_changes: true,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LoadAssetsPlugin)
        .add_plugin(MapLoaderPlugin)
        .add_plugin(KayakPlugin)
        .add_plugin(PathFindingServerPlugin)
        .add_plugin(GlobalEventPlugin)
        .add_plugin(SaveLoadFactoryPlugin)
        .add_plugin(TimersPlugin)
        .add_plugin(TimeSystemPlugin)
        .add_plugin(ControllablePlugin)
        .add_plugin(FaceAxisPlugin)
        .add_plugin(CameraSetupPlugin)
        .add_plugin(PanCameraPlugin) // Debug Purpose
        .add_plugin(ShadersPlugin)
        .add_plugin(EntityFactoryPlugin)
        .add_plugin(SceneManagerPlugin)
        .run();
}
