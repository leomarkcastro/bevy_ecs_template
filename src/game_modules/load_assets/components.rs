use bevy::{
    ecs::system::EntityCommands,
    prelude::{Component, Vec2},
    time::{Timer, TimerMode},
    utils::Uuid,
};

use crate::entity_factory::entities::global::animated::components::AnimatedComponent;

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

pub struct AnimationSettings {
    pub frame_duration: f32,
    pub index_start: u32,
    pub index_end: u32,
    pub direction: u32,
}

impl Default for AnimationSettings {
    fn default() -> Self {
        Self {
            frame_duration: 0.05,
            index_start: 0,
            index_end: 0,
            direction: 1,
        }
    }
}

pub fn insert_animation_components(
    ent_com: &mut EntityCommands,
    animation_settings: Option<AnimationSettings>,
) {
    let animation_settings = animation_settings.unwrap_or(AnimationSettings::default());
    ent_com
        .insert(Animated)
        .insert(AnimationTimer::new(animation_settings.frame_duration))
        .insert(AnimatedComponent {
            index_start: animation_settings.index_start,
            index_end: animation_settings.index_end,
            direction: animation_settings.direction,
        });
}
