// To describe how the Shaders component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::simple_point_light::systems::SimplePointLightPlugin;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SimplePointLightPlugin);
    }
}
