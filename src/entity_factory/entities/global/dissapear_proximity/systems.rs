// To describe how the DissapearProximity component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::{DissapearProximityComponent, DissapearProximityEntity};

pub struct DissapearProximityPlugin;

impl Plugin for DissapearProximityPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(dissapear_proximity_init_system)
            .add_system(dissapear_proximity_system);
    }
}

fn dissapear_proximity_init_system(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            ..Default::default()
        })
        .insert(DissapearProximityEntity)
        .insert(DissapearProximityComponent {
            data: "Hello, World!".to_string(),
            printed: false,
        });
}

fn dissapear_proximity_system(
    mut query: Query<
        &mut DissapearProximityComponent,
        With<DissapearProximityEntity>,
    >,
) {
    // Single Query
    if let Ok(mut dissapear_proximity_component) = query.get_single_mut() {
        dissapear_proximity_component.data = "Hello, World!".to_string();
    }

    // Multiple Queries
    for mut dissapear_proximity_component in query.iter_mut() {
        if (dissapear_proximity_component.printed) {
            continue;
        }

        println!("{:?}", dissapear_proximity_component.data);
        dissapear_proximity_component.printed = true;
    }
}
