use bevy::prelude::Component;

// To be used as data for the controllable entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Debug)]
pub struct ButtonClick {
    pub hold: bool,
    pub pressed: bool,
}

#[derive(Component, Debug)]
pub struct ControllableComponent {
    // system
    pub enabled: bool, // To enable/disable the controllable component

    // movement
    pub joy_x: f32, // X-axis
    pub joy_y: f32, // Y-axis

    // interactions
    pub btn_a: ButtonClick, // On our context, mainly to shoot
    pub btn_b: ButtonClick, // On our context, mainly to interact with world
    pub btn_c: ButtonClick, // On our context, mainly to cancel
    pub btn_d: ButtonClick, // On our context, mainly to pause

                            // we can add more buttons here
}

impl ControllableComponent {
    pub fn new(is_enabled: bool) -> Self {
        Self {
            joy_x: 0.0,
            joy_y: 0.0,
            btn_a: ButtonClick {
                hold: false,
                pressed: false,
            },
            btn_b: ButtonClick {
                hold: false,
                pressed: false,
            },
            btn_c: ButtonClick {
                hold: false,
                pressed: false,
            },
            btn_d: ButtonClick {
                hold: false,
                pressed: false,
            },
            enabled: is_enabled,
        }
    }
}
