use bevy::{prelude::Component, utils::Uuid};

use crate::{
    entity_factory::entities::global::ai::components::AITeam,
    game_modules::global_event::systems::GlobalEvent,
};

// To be used as data for the proximity entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Component, Clone)]
pub struct ProximityDataComponent {
    pub id: Uuid,
    pub triggers_event: bool,
    pub proximity_distance: f32,
    pub triggerer_type: AITeam,
    pub triggered_by: AITeam,
    pub trigger_event: GlobalEvent,
}

impl Default for ProximityDataComponent {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            triggers_event: false,
            proximity_distance: 0.0,
            triggerer_type: AITeam::Pickupable,
            triggered_by: AITeam::Player,
            trigger_event: GlobalEvent {
                event_data: "".to_string(),
                scene_id: "".to_string(),
            },
        }
    }
}
