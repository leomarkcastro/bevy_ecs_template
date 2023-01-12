use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::gui::crop_image::CropImageBundle;

use super::{
    bordered::{gui_plug_bordered_widget, BorderedBundle},
    crop_image::gui_plug_crop_image,
};

// =========================== INTERNALLY ABSTRACTED DEFINITIONS

#[derive(Component, Default, PartialEq, Clone)]
pub struct Dialogue;

impl Widget for Dialogue {}

fn status_hud_renderer(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    status_data: Res<DialogueResource>,
    asset_server: Res<AssetServer>,
) -> bool {
    let parent_id = Some(entity);
    let state = status_data;
    // let heart = asset_server.load("image_gui/images/icon_mono_health.png");
    rsx! {
        <BorderedBundle
            styles={KStyle{
                layout_type: LayoutType::Row.into(),
                col_between: Units::Pixels(0.).into(),
                padding: Edge::all(Units::Pixels(7.5)).into(),
                background_color: Color::rgba(0.5, 0.5, 0.5, 0.75).into(),
                // max_height: Units::Pixels(200.0).into(),
                // max_width: Units::Pixels(200.0).into(),
                // row_index: 4.into(),
                // col_index: 4.into(),
                ..default()
            }}
        >
            <ScrollContextProviderBundle>
                <ScrollBoxBundle
                    styles={KStyle{
                        layout_type: LayoutType::Column.into(),
                        row_between: Units::Pixels(5.0).into(),
                        // row_index: 4.into(),
                        // col_index: 4.into(),
                        ..default()
                    }}
                >
                    <TextWidgetBundle
                        text={
                            TextProps {
                                content: "asa".into(),
                                size: 15.0,
                                alignment: Alignment::End.into(),
                                ..Default::default()
                            }
                        }
                        styles={
                            KStyle {
                                color: Color::ORANGE_RED.into(),
                                padding_bottom: StyleProp::Value(Units::Pixels(5.)),
                                ..Default::default()
                            }
                        }
                    />

                    <TextWidgetBundle
                        text={
                            TextProps {
                                content: "asfadsf4sd65af4 1a6d5f4 sa6df54 as16f5d4 as6531as53dg4 asd65g 41asd35g".into(),
                                size: 12.0,
                                alignment: Alignment::End.into(),
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
                    <TextWidgetBundle
                        text={
                            TextProps {
                                content: "- Fetch Food".into(),
                                size: 12.0,
                                alignment: Alignment::End.into(),
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
                    <TextWidgetBundle
                        text={
                            TextProps {
                                content: "- Fetch Food".into(),
                                size: 12.0,
                                alignment: Alignment::End.into(),
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
                </ScrollBoxBundle>
            </ScrollContextProviderBundle>

        </BorderedBundle>
    };

    true
}

// =========================== EXTERNALLY USED DEFINITIONS

#[derive(Resource, Default)]
pub struct DialogueResource {
    pub messages: Vec<String>,
}

// Our own version of widget_update that handles resource change events.
pub fn widget_update_with_resource<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((widget_context, entity, previous_entity)): In<(KayakWidgetContext, Entity, Entity)>,
    status_data: Res<DialogueResource>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    status_data.is_changed()
}

pub fn gui_plug_dialogue(widget_context: &mut KayakRootContext, command: &mut Commands) {
    command.insert_resource(DialogueResource::default());

    widget_context.add_widget_data::<Dialogue, EmptyState>();
    widget_context.add_widget_system(
        Dialogue::default().get_name(),
        widget_update_with_resource::<Dialogue, EmptyState>,
        status_hud_renderer,
    );
}

#[derive(Bundle)]
pub struct DialogueBundle {
    pub dialogue_hud: Dialogue,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
}

impl Default for DialogueBundle {
    fn default() -> Self {
        Self {
            dialogue_hud: Dialogue::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            widget_name: Dialogue::default().get_name(),
        }
    }
}
