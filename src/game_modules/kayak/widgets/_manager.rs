// To describe how the Scene04 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    game_modules::timers::components::{OneSecondTimer, ThreeSecondTimer},
    gui::{notifications::NotificationsResource, tasks::TasksResource},
};

use super::_managerwidget::{gui_inject_manager_widget, WidgetManagerResource};

pub struct WidgetManagerPlugin;

impl Plugin for WidgetManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(gui_inject_manager_widget)
            .add_system(notif_test);
    }
}

fn notif_test(
    mut notif_res: ResMut<NotificationsResource>,
    mut tasks_res: ResMut<TasksResource>,
    time: Res<Time>,
    mut tick_time: ResMut<ThreeSecondTimer>,
) {
    if (tick_time.event_timer.tick(time.delta()).finished()) {
        if (notif_res.messages.len() > 4) {
            notif_res.messages.remove(0);
        }
        notif_res
            .messages
            .push(format!("Test message {}", time.elapsed_seconds_f64()));
        // pop the first message
        if (tasks_res.messages.len() > 4) {
            tasks_res.messages.remove(0);
        }
        tasks_res
            .messages
            .push(format!("Test message {}", time.elapsed_seconds_f64()));
    }
}
