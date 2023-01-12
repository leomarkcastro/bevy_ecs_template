// To describe how the Kayak component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use super::widgets::_manager::WidgetManagerPlugin;

pub struct KayakPlugin;

impl Plugin for KayakPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_plugin(WidgetManagerPlugin);
    }
}
