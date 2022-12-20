// To describe how the Proximity component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::{ProximityComponent, ProximityEntity};

pub struct ProximityPlugin;

impl Plugin for ProximityPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(proximity_init_system)
            .add_system(proximity_system);
    }
}

fn proximity_init_system(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            ..Default::default()
        })
        .insert(ProximityEntity)
        .insert(ProximityComponent {
            data: "Hello, World!".to_string(),
            printed: false,
        });
}

fn proximity_system(
    mut query: Query<
        &mut ProximityComponent,
        With<ProximityEntity>,
    >,
) {
    // Single Query
    if let Ok(mut proximity_component) = query.get_single_mut() {
        proximity_component.data = "Hello, World!".to_string();
    }

    // Multiple Queries
    for mut proximity_component in query.iter_mut() {
        if (proximity_component.printed) {
            continue;
        }

        println!("{:?}", proximity_component.data);
        proximity_component.printed = true;
    }
}
