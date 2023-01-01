// To describe how the GlobalEvent component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

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
        app.add_event::<GlobalEvent>();
    }
}
