// To describe how the SaveLoad component/entity should behave.
// WILL: contain pure logic that interacts with the component

use std::fs::*;
use std::io::Write;

use bevy::{
    prelude::*,
    tasks::{IoTaskPool, Task},
};

use crate::game_modules::save_load::data::{deserialize_decrypt, serialize_encrypt};

use super::data::GlobalSaveData;

#[derive(Debug)]
pub struct TriggerSaveLoadEvevnt {
    pub save: bool,
    pub load: bool,
    pub data: Option<GlobalSaveData>,
}

impl Default for TriggerSaveLoadEvevnt {
    fn default() -> Self {
        Self {
            save: false,
            load: false,
            data: None,
        }
    }
}

#[derive(Resource, Debug)]
pub struct SaveLoadState {
    is_saving: bool,
    save_task: Option<Task<()>>,
}

pub struct SaveLoadFactoryPlugin;

impl Plugin for SaveLoadFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TriggerSaveLoadEvevnt>()
            .insert_resource(SaveLoadState {
                is_saving: false,
                save_task: None,
            })
            .add_system(load_system)
            .add_system(save_system);
    }
}

// The new, updated scene data will be saved here so that you can see the changes
const SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";

fn load_system(
    mut commands: Commands,
    mut saveload_state: ResMut<SaveLoadState>,
    mut saveload_events: EventReader<TriggerSaveLoadEvevnt>,
) {
    let event = saveload_events.iter().last();

    match event {
        Some(event) if event.load => {
            if saveload_state.is_saving {
                println!("[On Load] Still Currently Saving... {:?}", saveload_state);
                return;
            }

            // This can't work in WASM as there is no filesystem access
            #[cfg(not(target_arch = "wasm32"))]
            IoTaskPool::get()
                .spawn(async move {
                    let data = read_to_string(format!("assets/{SCENE_FILE_PATH}"))
                        .expect("Unable to read file");

                    let loaded_data = deserialize_decrypt(&data);

                    let val = loaded_data.player_health;
                    println!("val: {}", val);
                })
                .detach();
        }
        _ => {}
    }
}

fn save_system(
    mut commands: Commands,
    mut saveload_state: ResMut<SaveLoadState>,
    mut saveload_events: EventReader<TriggerSaveLoadEvevnt>,
) {
    let event = saveload_events.iter().last();

    if saveload_state.save_task.is_some() {
        if saveload_state.save_task.as_ref().unwrap().is_finished() {
            saveload_state.is_saving = false;
            saveload_state.save_task = None;
            println!("[Save] Finished Saving");
        }
    }
    match event {
        Some(event) if event.save => {
            if saveload_state.is_saving {
                println!("[Save] Already Saving... {:?}", saveload_state);
                return;
            }

            println!("[Save] Start Saving...");

            let ref_data = event.data.as_ref().unwrap();

            let data_string = serialize_encrypt(ref_data);

            saveload_state.is_saving = true;

            // This can't work in WASM as there is no filesystem access
            #[cfg(not(target_arch = "wasm32"))]
            let task = IoTaskPool::get().spawn(async move {
                // Write the scene RON data to file
                File::create(format!("assets/{SCENE_FILE_PATH}"))
                    .and_then(|mut file| file.write(data_string.as_bytes()))
                    .expect("Error while writing scene to file");
            });

            saveload_state.save_task = Some(task);
        }
        _ => {}
    }
}
