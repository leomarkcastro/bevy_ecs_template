// To describe how the Proximity component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::game_modules::global_event::systems::GlobalEvent;

use super::components::ProximityDataComponent;

pub struct ProximityPlugin;

impl Plugin for ProximityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(proximity_system);
    }
}

fn proximity_init_system() {}

fn proximity_system(
    prox_query: Query<(&ProximityDataComponent, &Transform)>,
    mut global_event_writer: EventWriter<GlobalEvent>,
) {
    for (proximity, transform) in prox_query.iter() {
        if (!proximity.triggers_event) {
            continue;
        }

        // query all players
        let b_agent_filter = prox_query
            .iter()
            .filter(|(identifier, _)| identifier.triggerer_type == proximity.triggered_by);

        // check if any player is within detection range
        for (identifier, &b_agent_transform) in b_agent_filter {
            let distance = transform
                .translation
                .truncate()
                .distance(b_agent_transform.translation.truncate());

            if distance < proximity.proximity_distance {
                global_event_writer.send(GlobalEvent {
                    event_data: proximity.trigger_event.event_data.clone(),
                    scene_id: proximity.trigger_event.scene_id.clone(),
                });
                break;
            }
        }
    }
}
