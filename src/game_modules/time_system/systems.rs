// To describe how the TimeSystem component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::game_modules::timers::components::{
    HalfMilliSecondTimer, MillisencondTimer, OneSecondTimer, ThreeSecondTimer,
};

pub struct TimeSystemPlugin;

#[derive(Resource)]
pub struct CurrentWorldTimeGlobal {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
    pub active: bool,
}

impl Default for CurrentWorldTimeGlobal {
    fn default() -> Self {
        CurrentWorldTimeGlobal {
            days: 0,
            hours: 8,
            minutes: 0,
            active: true,
        }
    }
}

impl Plugin for TimeSystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentWorldTimeGlobal>()
            .add_system(time_ticking_system);
    }
}
fn time_ticking_system(
    mut time_tick: ResMut<CurrentWorldTimeGlobal>,
    time: Res<Time>,
    mut tick_time: ResMut<OneSecondTimer>,
) {
    if (!time_tick.active) {
        return;
    }
    if (tick_time.event_timer.tick(time.delta()).finished()) {
        time_tick.minutes += 3;
        if (time_tick.minutes >= 60) {
            time_tick.hours += 1;
            time_tick.minutes = 0;
        }
        if (time_tick.hours >= 24) {
            time_tick.days += 1;
            time_tick.hours = 0;
        }
        // println!(
        //     "[Day {}] {}:{}",
        //     time_tick.days, time_tick.hours, time_tick.minutes
        // )
    }
}
