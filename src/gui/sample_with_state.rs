use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::game_modules::time_system::systems::CurrentWorldTimeGlobal;

// =========================== INTERNALLY ABSTRACTED DEFINITIONS

#[derive(Component, Default, PartialEq, Clone)]
pub struct SampleWidget;

impl Widget for SampleWidget {}

#[derive(Component, Default, PartialEq, Clone)]
pub struct SampleWidgetState {
    pub foo: u32,
}

fn sample_widget_renderer(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    query: Query<&SampleWidgetState>,
    current_time: Res<CurrentWorldTimeGlobal>,
) -> bool {
    let state_entity =
        widget_context.use_state(&mut commands, entity, SampleWidgetState::default());

    if let Ok(current_count) = query.get(state_entity) {
        let parent_id = Some(entity);
        rsx! {
            <ElementBundle>
                <TextWidgetBundle
                    text={
                        TextProps {
                            content: format!("Current Count: {} [{}: {}]", current_count.foo, current_time.hours, current_time.minutes),
                            size: 16.0,
                            line_height: Some(40.0),
                            ..Default::default()
                        }
                    }
                />
                <KButtonBundle
                    button={KButton {
                        text: "Click me!".into(),
                        ..Default::default()
                    }}
                    on_event={OnEvent::new(
                        move |
                          In((event_dispatcher_context, _, mut event, _entity)): In<(EventDispatcherContext, WidgetState, Event, Entity)>,
                          mut query: Query<&mut SampleWidgetState>
                        |
                        {
                            match event.event_type {
                                EventType::Click(..) => {
                                    event.prevent_default();
                                    event.stop_propagation();
                                    if let Ok(mut current_count) = query.get_mut(state_entity) {
                                        current_count.foo += 1;
                                    }
                                }

                                _ => {}
                            }
                            (event_dispatcher_context, event)
                        },
                    )}
                />
            </ElementBundle>
        };
    }

    true
}

// =========================== EXTERNALLY USED DEFINITIONS

// Our own version of widget_update that handles resource change events.
pub fn widget_update_with_resource<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((widget_context, entity, previous_entity)): In<(KayakWidgetContext, Entity, Entity)>,
    current_time: Res<CurrentWorldTimeGlobal>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    widget_param.has_changed(&widget_context, entity, previous_entity) || current_time.is_changed()
}

pub fn gui_plug_sample_widget(widget_context: &mut KayakRootContext) {
    widget_context.add_widget_data::<SampleWidget, SampleWidgetState>();
    widget_context.add_widget_system(
        SampleWidget::default().get_name(),
        widget_update_with_resource::<SampleWidget, SampleWidgetState>,
        // widget_update::<SampleWidget, SampleWidgetState>,
        sample_widget_renderer,
    );
}

#[derive(Bundle)]
pub struct SampleWidgetBundle {
    pub count: SampleWidget,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
}

impl Default for SampleWidgetBundle {
    fn default() -> Self {
        Self {
            count: SampleWidget::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            widget_name: SampleWidget::default().get_name(),
        }
    }
}
