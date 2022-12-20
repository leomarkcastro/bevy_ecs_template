use bevy::{
    prelude::{Component, Resource},
    time::{Timer, TimerMode},
};

// To be used as data for the timers entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Resource)]
pub struct OneSecondTimer {
    pub event_timer: Timer,
}

impl Default for OneSecondTimer {
    fn default() -> Self {
        OneSecondTimer {
            event_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}
#[derive(Resource)]
pub struct ThreeSecondTimer {
    pub event_timer: Timer,
}

impl Default for ThreeSecondTimer {
    fn default() -> Self {
        ThreeSecondTimer {
            event_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        }
    }
}
