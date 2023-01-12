use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::gui::crop_image::CropImageBundle;

use super::{
    bordered::{gui_plug_bordered_widget, BorderedBundle},
    crop_image::gui_plug_crop_image,
};

// =========================== INTERNALLY ABSTRACTED DEFINITIONS

#[derive(Component, Default, PartialEq, Clone)]
pub struct Notifications;

impl Widget for Notifications {}

fn status_hud_renderer(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    status_data: Res<NotificationsResource>,
) -> bool {
    let parent_id = Some(entity);
    let state = status_data;
    rsx! {
        <ElementBundle
            styles={KStyle{
                layout_type: LayoutType::Column.into(),
                row_between: Units::Pixels(1.0).into(),
                padding: Edge::axis(Units::Pixels(2.5), Units::Pixels(0.5)).into(),
                // row_index: 4.into(),
                // col_index: 4.into(),
                ..default()
            }}
        >
            {
                {for i in (0..state.messages.len()) {
                    constructor! {
                        <TextWidgetBundle
                            text={
                                TextProps {
                                    content: state.messages[i].clone(),
                                    size: 12.0,
                                    ..Default::default()
                                }
                            }
                            styles={
                                KStyle {
                                    color: Color::ORANGE_RED.into(),
                                    ..Default::default()
                                }
                            }
                        />
                    }
                }}
            }

        </ElementBundle>
    };

    true
}

// =========================== EXTERNALLY USED DEFINITIONS

#[derive(Resource, Default)]
pub struct NotificationsResource {
    pub messages: Vec<String>,
}

// Our own version of widget_update that handles resource change events.
pub fn widget_update_with_resource<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((widget_context, entity, previous_entity)): In<(KayakWidgetContext, Entity, Entity)>,
    status_data: Res<NotificationsResource>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    status_data.is_changed()
}

pub fn gui_plug_notifications(widget_context: &mut KayakRootContext, command: &mut Commands) {
    command.insert_resource(NotificationsResource::default());

    widget_context.add_widget_data::<Notifications, EmptyState>();
    widget_context.add_widget_system(
        Notifications::default().get_name(),
        widget_update_with_resource::<Notifications, EmptyState>,
        status_hud_renderer,
    );
}

#[derive(Bundle)]
pub struct NotificationsBundle {
    pub notifications_hud: Notifications,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
}

impl Default for NotificationsBundle {
    fn default() -> Self {
        Self {
            notifications_hud: Notifications::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            widget_name: Notifications::default().get_name(),
        }
    }
}
