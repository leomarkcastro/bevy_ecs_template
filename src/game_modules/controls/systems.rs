// To describe how the Controls component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::{ControlsComponent, ControlsEntity};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(controls_init_system)
            .add_system(controls_system);
    }
}

fn controls_init_system(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            ..Default::default()
        })
        .insert(ControlsEntity)
        .insert(ControlsComponent {
            data: "Hello, World!".to_string(),
            printed: false,
        });
}

fn controls_system(mut query: Query<&mut ControlsComponent, With<ControlsEntity>>) {
    // Single Query
    if let Ok(mut controlscomponent) = query.get_single_mut() {
        controlscomponent.data = "Hello, World!".to_string();
    }

    // Multiple Queries
    for mut controlscomponent in query.iter_mut() {
        if (controlscomponent.printed) {
            continue;
        }

        println!("{:?}", controlscomponent.data);
        controlscomponent.printed = true;
    }
}
