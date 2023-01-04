// To describe how the GlobalEvent component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::entity_factory::entities::{
    global::proximity::components::ProximityDataComponent,
    pickupablev1::entities::Pickupablev1Entity,
};

// This is an attempt to create a global event that can be triggered by any entity.
// Note: Avoid using this as much as possible. This is a last resort.
#[derive(Debug, Clone)]
pub struct GlobalEvent {
    pub event_data: String,
    pub scene_id: String,
}

pub struct GlobalEventPlugin;

impl Plugin for GlobalEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GlobalEvent>()
            .add_system(globalevent_print_system);
    }
}

fn globalevent_print_system(mut command: Commands, mut global_even_read: EventReader<GlobalEvent>) {
    let scene_events = global_even_read.iter().filter(|e| e.scene_id == "PRINT");
    for event in scene_events {
        // get current timestamp
        let now = std::time::SystemTime::now();
        println!("[{:?}] GlobalEvent: {}", now, event.event_data);
    }
}
