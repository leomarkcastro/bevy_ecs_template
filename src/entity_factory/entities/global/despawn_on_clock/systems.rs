// To describe how the Despawn component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::{
    game_modules::timers::components::ThreeSecondTimer,
    utils::check_collide::{check_2circle_collide, CircleCollideData},
};

use super::components::DespawnWithTimerComponent;

#[derive(Resource)]
pub struct DespawnTrackerGlobal {
    pub spawned_id: Vec<String>,
}

impl Default for DespawnTrackerGlobal {
    fn default() -> Self {
        Self { spawned_id: vec![] }
    }
}

pub struct DespawnWithTimerPlugin;

impl Plugin for DespawnWithTimerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DespawnTrackerGlobal::default())
            .add_system(despawn_system);
    }
}

fn despawn_system(
    mut command: Commands,
    mut query: Query<(Entity, &DespawnWithTimerComponent, &Transform)>,
    mut despawn_tracker: ResMut<DespawnTrackerGlobal>,

    time: Res<Time>,
    mut three_sec_timer: ResMut<ThreeSecondTimer>,
) {
    if !three_sec_timer.event_timer.tick(time.delta()).finished() {
        return;
    }

    // Multiple Queries
    for despawn_component in query.iter_mut() {
        let (entity, mut despawn_component, transform) = despawn_component;

        // despawn entity
        // println!("Despawning entity: {}", despawn_component.id);
        // remove id from tracker
        if (despawn_component.despawn_on < time.elapsed_seconds()) {
            despawn_tracker
                .spawned_id
                .retain(|id| id != &despawn_component.id);
            command.entity(entity).despawn_recursive();
        }
    }
}
