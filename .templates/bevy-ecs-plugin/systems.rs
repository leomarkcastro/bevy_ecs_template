// To describe how the __templateNameToPascalCase__ component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::{__templateNameToPascalCase__Component, __templateNameToPascalCase__Entity};

pub struct __templateNameToPascalCase__Plugin;

impl Plugin for __templateNameToPascalCase__Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(__templateNameToLowerCase___init_system)
            .add_system(__templateNameToLowerCase___system);
    }
}

fn __templateNameToLowerCase___init_system(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            ..Default::default()
        })
        .insert(__templateNameToPascalCase__Entity)
        .insert(__templateNameToPascalCase__Component {
            data: "Hello, World!".to_string(),
            printed: false,
        });
}

fn __templateNameToLowerCase___system(
    mut query: Query<
        &mut __templateNameToPascalCase__Component,
        With<__templateNameToPascalCase__Entity>,
    >,
) {
    // Single Query
    if let Ok(mut __templateNameToLowerCase___component) = query.get_single_mut() {
        __templateNameToLowerCase___component.data = "Hello, World!".to_string();
    }

    // Multiple Queries
    for mut __templateNameToLowerCase___component in query.iter_mut() {
        if (__templateNameToLowerCase___component.printed) {
            continue;
        }

        println!("{:?}", __templateNameToLowerCase___component.data);
        __templateNameToLowerCase___component.printed = true;
    }
}
