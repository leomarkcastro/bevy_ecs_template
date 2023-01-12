use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::gui::crop_image::CropImageBundle;

use super::{
    bordered::{gui_plug_bordered_widget, BorderedBundle},
    crop_image::gui_plug_crop_image,
};

// =========================== INTERNALLY ABSTRACTED DEFINITIONS

#[derive(Component, Default, PartialEq, Clone)]
pub struct MenuOverview;

impl Widget for MenuOverview {}

fn status_hud_renderer(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    status_data: Res<MenuOverviewResource>,
    asset_server: Res<AssetServer>,
) -> bool {
    let parent_id = Some(entity);
    let state = status_data;
    let heart = asset_server.load("image_gui/images/icon_mono_health.png");
    let armor = asset_server.load("image_gui/images/icon_mono_armor.png");
    let weight = asset_server.load("image_gui/images/icon_mono_weight.png");
    rsx! {
        <BorderedBundle>
            <ElementBundle
                styles={KStyle{
                padding: Edge::all(Units::Pixels(7.5)).into(),
                layout_type: LayoutType::Grid.into(),
                grid_rows: vec![Units::Stretch(2.0), Units::Stretch(5.0)].into(),
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
                    <ElementBundle
                        styles={KStyle{
                            layout_type: LayoutType::Row.into(),
                            col_between: Units::Pixels(15.0).into(),
                            top: Units::Stretch(1.0).into(),
                            ..default()
                        }}
                    >
                        <KImageBundle
                            image={KImage(heart)}
                            styles={KStyle {
                                width: StyleProp::Value(Units::Pixels(20.)),
                                height: Units::Stretch(1.0).into(),
                                ..Default::default()
                            }}
                        />
                        <CropImageBundle styles= {
                            KStyle {
                                width: Units::Stretch(1.0).into(),
                                height: Units::Stretch(1.0).into(),
                                line_height: 10.0.into(),
                                ..Default::default()
                            }
                        } />
                    </ElementBundle>
                    <ElementBundle
                        styles={KStyle{
                            layout_type: LayoutType::Row.into(),
                            col_between: Units::Pixels(15.0).into(),
                            ..default()
                        }}
                    >
                        <KImageBundle
                            image={KImage(armor)}
                            styles={KStyle {
                                width: StyleProp::Value(Units::Pixels(20.)),
                                height: Units::Stretch(1.0).into(),
                                ..Default::default()
                            }}
                        />
                        <CropImageBundle styles= {
                            KStyle {
                                width: Units::Stretch(1.0).into(),
                                height: Units::Stretch(1.0).into(),
                                line_height: 10.0.into(),
                                ..Default::default()
                            }
                        } />
                    </ElementBundle>
                    <ElementBundle
                        styles={KStyle{
                            layout_type: LayoutType::Row.into(),
                            col_between: Units::Pixels(15.0).into(),
                            ..default()
                        }}
                    >
                        <KImageBundle
                            image={KImage(weight)}
                            styles={KStyle {
                                width: StyleProp::Value(Units::Pixels(20.)),
                                height: Units::Stretch(1.0).into(),
                                ..Default::default()
                            }}
                        />
                        <CropImageBundle styles= {
                            KStyle {
                                width: Units::Stretch(1.0).into(),
                                height: Units::Stretch(1.0).into(),
                                line_height: 10.0.into(),
                                ..Default::default()
                            }
                        } />
                    </ElementBundle>
                </ElementBundle>
                <ElementBundle
                    styles={KStyle{
                    col_index: 0.into(),
                    row_index: 1.into(),
                    row_between: Units::Pixels(7.0).into(),
                    layout_type: LayoutType::Column.into(),

                    ..default()
                }}>

                    <ScrollContextProviderBundle>
                        <ScrollBoxBundle>
                            <TextWidgetBundle
                                text={
                                    TextProps {
                                        content: "Survival Info".into(),
                                        size: 20.0,
                                        ..Default::default()
                                    }
                                }
                                styles={
                                    KStyle {
                                        color: Color::ORANGE_RED.into(),
                                        top: Units::Pixels(15.0).into(),
                                        ..Default::default()
                                    }
                                }
                            />
                            <TextWidgetBundle
                                text={
                                    TextProps {
                                        content: format!("{}:{}:{}", 0, 0, 0).into(),
                                        size: 15.0,
                                        ..Default::default()
                                    }
                                }
                                styles={
                                    KStyle {
                                        color: Color::ORANGE_RED.into(),
                                        top: Units::Pixels(15.0).into(),
                                        ..Default::default()
                                    }
                                }
                            />
                            <TextWidgetBundle
                                text={
                                    TextProps {
                                        content: "Current Tasks".into(),
                                        size: 20.0,
                                        ..Default::default()
                                    }
                                }
                                styles={
                                    KStyle {
                                        color: Color::ORANGE_RED.into(),
                                        top: Units::Pixels(15.0).into(),
                                        ..Default::default()
                                    }
                                }
                            />
                            {
                                {for i in (0..10) {
                                    constructor! {
                                        <TextWidgetBundle
                                            text={
                                                TextProps {
                                                    content: format!("{}", "Hello World!").into(),
                                                    size: 15.0,
                                                    ..Default::default()
                                                }
                                            }
                                            styles={
                                                KStyle {
                                                    color: Color::ORANGE_RED.into(),
                                                    top: Units::Pixels(15.0).into(),
                                                    ..Default::default()
                                                }
                                            }
                                        />
                                    }
                                }}
                            }
                        </ScrollBoxBundle>
                    </ScrollContextProviderBundle>
                </ElementBundle>
            </ElementBundle>
        </BorderedBundle>
    };

    true
}

// =========================== EXTERNALLY USED DEFINITIONS

#[derive(Resource, Default)]
pub struct MenuOverviewResource {
    pub messages: Vec<String>,
}

// Our own version of widget_update that handles resource change events.
pub fn widget_update_with_resource<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((widget_context, entity, previous_entity)): In<(KayakWidgetContext, Entity, Entity)>,
    status_data: Res<MenuOverviewResource>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    status_data.is_changed()
}

pub fn gui_plug_menuoverview(widget_context: &mut KayakRootContext, command: &mut Commands) {
    command.insert_resource(MenuOverviewResource::default());

    widget_context.add_widget_data::<MenuOverview, EmptyState>();
    widget_context.add_widget_system(
        MenuOverview::default().get_name(),
        widget_update_with_resource::<MenuOverview, EmptyState>,
        status_hud_renderer,
    );
}

#[derive(Bundle)]
pub struct MenuOverviewBundle {
    pub menuoverview_hud: MenuOverview,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
}

impl Default for MenuOverviewBundle {
    fn default() -> Self {
        Self {
            menuoverview_hud: MenuOverview::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            widget_name: MenuOverview::default().get_name(),
        }
    }
}
