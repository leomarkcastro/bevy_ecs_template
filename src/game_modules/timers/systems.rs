// To describe how the Timers component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::{components::ThreeSecondTimer, OneSecondTimer};

pub struct TimersPlugin;

impl Plugin for TimersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OneSecondTimer>()
            .init_resource::<ThreeSecondTimer>();
    }
}
