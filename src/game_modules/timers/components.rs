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

#[derive(Resource)]
pub struct QuarterSencondTimer {
    pub event_timer: Timer,
}

impl Default for QuarterSencondTimer {
    fn default() -> Self {
        QuarterSencondTimer {
            event_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        }
    }
}
#[derive(Resource)]
pub struct MillisencondTimer {
    pub event_timer: Timer,
}

impl Default for MillisencondTimer {
    fn default() -> Self {
        MillisencondTimer {
            event_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct HalfMilliSecondTimer {
    pub event_timer: Timer,
}

impl Default for HalfMilliSecondTimer {
    fn default() -> Self {
        HalfMilliSecondTimer {
            event_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
        }
    }
}
