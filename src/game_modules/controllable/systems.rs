// To describe how the Controllable component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::{components::ButtonClick, ControllableEntity, ControllableResource};

pub struct ControllablePlugin;

impl Plugin for ControllablePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControllableResource::new(true))
            // .add_startup_system(controllable_test_init_system)
            .add_system(controllable_mouse_system)
            .add_system(controllable_kb_system);
        // .add_system(controllable_view_system);
    }
}

fn controllable_test_init_system(mut commands: Commands) {
    // commands
    //     .spawn(SpriteBundle {
    //         ..Default::default()
    //     })
    //     .insert(ControllableEntity)
    //     .insert(ControllableComponent::new(true));
}

fn controllable_kb_system(
    kb: Res<Input<KeyCode>>,
    mut controllable_component: ResMut<ControllableResource>,
) {
    // We can either control ourselves, a car, a crane or anything else.
    if (!controllable_component.enabled) {
        return;
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

    // key_to_kb(&mut controllable_component.btn_a, KeyCode::Space);
    // key_to_kb(&mut controllable_component.btn_b, KeyCode::E);
    key_to_kb(&mut controllable_component.btn_c, KeyCode::Escape);
    key_to_kb(&mut controllable_component.btn_d, KeyCode::Return);
}

fn controllable_mouse_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut controllable_component: ResMut<ControllableResource>,
) {
    // We can either control ourselves, a car, a crane or anything else.
    if (!controllable_component.enabled) {
        return;
    }

    let key_to_kb = |mut key: &mut ButtonClick, kbc: MouseButton| {
        key.hold = mouse_button_input.pressed(kbc);
        key.pressed = mouse_button_input.just_pressed(kbc);
    };

    key_to_kb(&mut controllable_component.btn_a, MouseButton::Left);
    key_to_kb(&mut controllable_component.btn_b, MouseButton::Right);
}

fn controllable_view_system(controllable_component: Res<ControllableResource>) {
    // We can either control ourselves, a car, a crane or anything else.

    if (!controllable_component.enabled) {
        return;
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
