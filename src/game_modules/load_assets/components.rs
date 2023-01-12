use bevy::{
    ecs::system::EntityCommands,
    prelude::{Component, Vec2},
    time::{Timer, TimerMode},
    utils::Uuid,
};

#[derive(Component, Debug, Clone)]
pub struct Animated;

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

impl AnimationTimer {
    pub fn new(frame_duration: f32) -> Self {
        Self(Timer::from_seconds(frame_duration, TimerMode::Repeating))
    }
}

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, TimerMode::Repeating))
    }
}

#[derive(Default)]
pub struct AnimationSettings {
    pub frame_duration: f32,
}

pub fn insert_animation_components(
    ent_com: &mut EntityCommands,
    animation_settings: Option<AnimationSettings>,
) {
    let frame = match animation_settings {
        Some(settings) => settings.frame_duration,
        None => 0.05,
    };
    ent_com.insert(Animated).insert(AnimationTimer::new(frame));
}
