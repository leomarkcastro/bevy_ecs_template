// To describe how the template component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::{components::TemplateComponent, entities::TemplateEntity};

pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(template_init_system)
            .add_system(template_system);
    }
}

fn template_init_system(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            ..Default::default()
        })
        .insert(TemplateEntity)
        .insert(TemplateComponent {
            data: "Hello, World!".to_string(),
            printed: false,
        });
}

fn template_system(mut query: Query<&mut TemplateComponent, With<TemplateEntity>>) {
    // Single Query
    if let Ok(mut template_component) = query.get_single_mut() {
        template_component.data = "Hello, World!".to_string();
    }

    // Multiple Queries
    for mut template_component in query.iter_mut() {
        if (template_component.printed) {
            continue;
        }

        println!("{:?}", template_component.data);
        template_component.printed = true;
    }
}
