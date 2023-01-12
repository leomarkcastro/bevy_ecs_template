use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::gui::crop_image::CropImageBundle;

use super::{
    bordered::{gui_plug_bordered_widget, BorderedBundle},
    crop_image::gui_plug_crop_image,
};

// =========================== INTERNALLY ABSTRACTED DEFINITIONS

#[derive(Component, Default, PartialEq, Clone)]
pub struct Tasks;

impl Widget for Tasks {}

fn status_hud_renderer(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    status_data: Res<TasksResource>,
) -> bool {
    let parent_id = Some(entity);
    let state = status_data;
    rsx! {
        <BackgroundBundle
            styles={KStyle{
                layout_type: LayoutType::Column.into(),
                row_between: Units::Pixels(1.0).into(),
                padding: Edge::axis(Units::Pixels(17.5), Units::Pixels(13.)).into(),
                left: Units::Stretch(1.0).into(),
                max_width: Units::Pixels(300.0).into(),
                max_height: Units::Pixels(150.0).into(),
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.15).into(),
                // row_index: 4.into(),
                // col_index: 4.into(),
                ..default()
            }}
        >
            <ScrollContextProviderBundle>
                <ScrollBoxBundle>
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
                </ScrollBoxBundle>
            </ScrollContextProviderBundle>

        </BackgroundBundle>
    };

    true
}

// =========================== EXTERNALLY USED DEFINITIONS

#[derive(Resource, Default)]
pub struct TasksResource {
    pub messages: Vec<String>,
}

// Our own version of widget_update that handles resource change events.
pub fn widget_update_with_resource<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((widget_context, entity, previous_entity)): In<(KayakWidgetContext, Entity, Entity)>,
    status_data: Res<TasksResource>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    status_data.is_changed()
}

pub fn gui_plug_tasks(widget_context: &mut KayakRootContext, command: &mut Commands) {
    command.insert_resource(TasksResource::default());

    widget_context.add_widget_data::<Tasks, EmptyState>();
    widget_context.add_widget_system(
        Tasks::default().get_name(),
        widget_update_with_resource::<Tasks, EmptyState>,
        status_hud_renderer,
    );
}

#[derive(Bundle)]
pub struct TasksBundle {
    pub tasks_hud: Tasks,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
}

impl Default for TasksBundle {
    fn default() -> Self {
        Self {
            tasks_hud: Tasks::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            widget_name: Tasks::default().get_name(),
        }
    }
}
