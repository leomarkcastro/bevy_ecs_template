// To describe how the Animated component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::AnimatedComponent;

pub struct AnimatedPlugin;

impl Plugin for AnimatedPlugin {
    fn build(&self, app: &mut App) {}
}
