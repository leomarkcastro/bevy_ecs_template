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
    prox_query: Query<(&ProximityDataComponent, &Transform, &GlobalTransform)>,
    mut global_event_writer: EventWriter<GlobalEvent>,
) {
    for (a_identifier, a_transform, a_gtransform) in prox_query.iter() {
        if (!a_identifier.triggers_event) {
            continue;
        }

        // query all players
        let b_agent_filter = prox_query.iter().filter(|(identifier, _, _)| {
            return identifier.triggerer_type == a_identifier.triggered_by;
        });

        // check if any player is within detection range
        for (b_identifier, &b_transform, b_gtransform) in b_agent_filter {
            // get the vec3 of a and b from global transform
            let a_gt = a_gtransform.to_scale_rotation_translation().2;
            let b_gt = b_gtransform.to_scale_rotation_translation().2;

            let distance = b_gt.truncate().distance(a_gt.truncate());

            // println!("{} {}", distance, a_identifier.proximity_distance);
            if distance < a_identifier.proximity_distance {
                global_event_writer.send(GlobalEvent {
                    event_data: a_identifier.trigger_event.event_data.clone(),
                    scene_id: a_identifier.trigger_event.scene_id.clone(),
                });
                break;
            }
        }
    }
}
