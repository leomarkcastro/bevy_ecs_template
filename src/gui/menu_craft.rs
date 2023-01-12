use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::gui::crop_image::CropImageBundle;

use super::{
    bordered::{gui_plug_bordered_widget, BorderedBundle},
    crop_image::gui_plug_crop_image,
};

// =========================== INTERNALLY ABSTRACTED DEFINITIONS

#[derive(Component, Default, PartialEq, Clone)]
pub struct MenuCraft;

impl Widget for MenuCraft {}

fn status_hud_renderer(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    status_data: Res<MenuCraftResource>,
    asset_server: Res<AssetServer>,
) -> bool {
    let parent_id = Some(entity);
    let state = status_data;
    let heart = asset_server.load("image_gui/images/icon_mono_health.png");
    // let armor = asset_server.load("image_gui/images/icon_mono_armor.png");
    // let weight = asset_server.load("image_gui/images/icon_mono_weight.png");
    rsx! {
        <BorderedBundle>
            <ElementBundle
                styles={KStyle{
                padding: Edge::all(Units::Pixels(7.5)).into(),
                layout_type: LayoutType::Grid.into(),
                grid_rows: vec![Units::Stretch(1.0), Units::Stretch(2.0), Units::Stretch(5.0), Units::Stretch(1.0)].into(),
                grid_cols: vec![Units::Stretch(1.0)].into(),
                ..default()
            }}>
                <ElementBundle
                    styles={KStyle{
                        col_index: 0.into(),
                        row_index: 0.into(),
                        row_between: Units::Pixels(7.0).into(),
                        ..default()
                }}>
                    <TextWidgetBundle
                        text={
                            TextProps {
                                content: "CRAFT".into(),
                                size: 25.0,
                                alignment: Alignment::Middle.into(),
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
                </ElementBundle>
                <ElementBundle
                    styles={KStyle{
                        col_index: 0.into(),
                        row_index: 1.into(),
                        row_between: Units::Pixels(7.0).into(),
                        ..default()
                }}>
                    <BorderedBundle>
                        <KImageBundle
                            image={KImage(heart.clone())}
                            styles={KStyle {
                                width: StyleProp::Value(Units::Pixels(40.)),
                                height: StyleProp::Value(Units::Pixels(40.)),
                                left: Units::Stretch(1.0).into(),
                                right: Units::Stretch(1.0).into(),
                                top: Units::Stretch(1.0).into(),
                                bottom: Units::Stretch(1.0).into(),
                                ..Default::default()
                            }}
                        />
                    </BorderedBundle>
                </ElementBundle>
                    <ElementBundle
                        styles={KStyle{
                            col_index: 0.into(),
                            row_index: 2.into(),
                            row_between: Units::Pixels(7.0).into(),
                            ..default()
                    }}>
                        <ScrollContextProviderBundle>
                            <ScrollBoxBundle>
                                <TextWidgetBundle
                                    text={
                                        TextProps {
                                            content: "Health  Pack".into(),
                                            size: 18.0,
                                            alignment: Alignment::Middle.into(),
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
                                            content: "Fresh from the mountains of clinic, this can heal you quicker".into(),
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
                                <TextWidgetBundle
                                    text={
                                        TextProps {
                                            content: "Recipe".into(),
                                            size: 14.0,
                                            alignment: Alignment::Middle.into(),
                                            ..Default::default()
                                        }
                                    }
                                    styles={
                                        KStyle {
                                            color: Color::ORANGE_RED.into(),
                                            top: StyleProp::Value(Units::Pixels(15.)),
                                            ..Default::default()
                                        }
                                    }
                                />
                                <TextWidgetBundle
                                    text={
                                        TextProps {
                                            content: "- Herbs".into(),
                                            size: 12.0,
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
                        </ScrollBoxBundle>
                    </ScrollContextProviderBundle>
                </ElementBundle>
                <ElementBundle
                    styles={KStyle{
                        col_index: 0.into(),
                        row_index: 3.into(),
                        row_between: Units::Pixels(7.0).into(),
                        ..default()
                }}>
                    <BorderedBundle>
                        <TextWidgetBundle
                            text={
                                TextProps {
                                    content: "CRAFT".into(),
                                    size: 25.0,
                                    alignment: Alignment::Middle.into(),
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
                    </BorderedBundle>
                </ElementBundle>
            </ElementBundle>
        </BorderedBundle>
    };

    true
}

// =========================== EXTERNALLY USED DEFINITIONS

#[derive(Resource, Default)]
pub struct MenuCraftResource {
    pub messages: Vec<String>,
}

// Our own version of widget_update that handles resource change events.
pub fn widget_update_with_resource<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((widget_context, entity, previous_entity)): In<(KayakWidgetContext, Entity, Entity)>,
    status_data: Res<MenuCraftResource>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    status_data.is_changed()
}

pub fn gui_plug_menucraft(widget_context: &mut KayakRootContext, command: &mut Commands) {
    command.insert_resource(MenuCraftResource::default());

    widget_context.add_widget_data::<MenuCraft, EmptyState>();
    widget_context.add_widget_system(
        MenuCraft::default().get_name(),
        widget_update_with_resource::<MenuCraft, EmptyState>,
        status_hud_renderer,
    );
}

#[derive(Bundle)]
pub struct MenuCraftBundle {
    pub menucraft_hud: MenuCraft,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
}

impl Default for MenuCraftBundle {
    fn default() -> Self {
        Self {
            menucraft_hud: MenuCraft::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            widget_name: MenuCraft::default().get_name(),
        }
    }
}
