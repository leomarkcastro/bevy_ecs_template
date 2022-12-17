// To describe how the Controllable component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::{components::ButtonClick, ControllableComponent, ControllableEntity};

pub struct ControllablePlugin;

impl Plugin for ControllablePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(controllable_test_init_system)
            .add_system(controllable_kb_system);
        // .add_system(controllable_view_system);
    }
}

fn controllable_test_init_system(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            ..Default::default()
        })
        .insert(ControllableEntity)
        .insert(ControllableComponent::new(true));
}

fn controllable_kb_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut ControllableComponent, With<ControllableEntity>>,
) {
    // We can either control ourselves, a car, a crane or anything else.
    for mut controllable_component in query.iter_mut() {
        if (!controllable_component.enabled) {
            continue;
        }

        if (kb.pressed(KeyCode::A)) {
            controllable_component.joy_x = -1.0;
        } else if (kb.pressed(KeyCode::D)) {
            controllable_component.joy_x = 1.0;
        } else {
            controllable_component.joy_x = 0.0;
        }

        if (kb.pressed(KeyCode::W)) {
            controllable_component.joy_y = 1.0;
        } else if (kb.pressed(KeyCode::S)) {
            controllable_component.joy_y = -1.0;
        } else {
            controllable_component.joy_y = 0.0;
        }

        let key_to_kb = |mut key: &mut ButtonClick, kbc: KeyCode| {
            key.hold = kb.pressed(kbc);
            key.pressed = kb.just_pressed(kbc);
        };

        key_to_kb(&mut controllable_component.btn_a, KeyCode::Space);
        key_to_kb(&mut controllable_component.btn_b, KeyCode::E);
        key_to_kb(&mut controllable_component.btn_c, KeyCode::Escape);
        key_to_kb(&mut controllable_component.btn_d, KeyCode::Return);
    }
}

fn controllable_view_system(query: Query<&ControllableComponent, With<ControllableEntity>>) {
    // We can either control ourselves, a car, a crane or anything else.
    for controllable_component in query.iter() {
        if (!controllable_component.enabled) {
            continue;
        }

        println!(
            "joy_x: {:?}, joy_y: {:?}, btn_a: {:?}, btn_b: {:?}, btn_c: {:?}, btn_d: {:?}",
            controllable_component.joy_x,
            controllable_component.joy_y,
            controllable_component.btn_a,
            controllable_component.btn_b,
            controllable_component.btn_c,
            controllable_component.btn_d
        );
    }
}
