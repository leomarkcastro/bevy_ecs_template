use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::gui::crop_image::CropImageBundle;

use super::{
    bordered::{gui_plug_bordered_widget, BorderedBundle},
    crop_image::gui_plug_crop_image,
};

// =========================== INTERNALLY ABSTRACTED DEFINITIONS

#[derive(Component, Default, PartialEq, Clone)]
pub struct MenuMenuInventory;

impl Widget for MenuMenuInventory {}

fn status_hud_renderer(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    status_data: Res<MenuMenuInventoryResource>,
    asset_server: Res<AssetServer>,
) -> bool {
    let parent_id = Some(entity);
    let state = status_data;
    let heart = asset_server.load("image_gui/images/icon_mono_health.png");
    rsx! {
        <BorderedBundle
            styles={KStyle{
                padding: Edge::all(Units::Pixels(7.5)).into(),
                ..default()
            }}
        >
            <ElementBundle
                styles={KStyle{
                    layout_type: LayoutType::Grid.into(),
                    grid_rows: vec![Units::Stretch(4.0), Units::Stretch(4.0), Units::Stretch(4.0), Units::Stretch(1.0)].into(),
                    grid_cols: vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0)].into(),
                    row_between: Units::Pixels(10.0).into(),
                    col_between: Units::Pixels(10.0).into(),
                    ..default()
                }}
            >
                {
                    {for i in (0..12) {
                        constructor! {
                            <ElementBundle
                                styles={KStyle{
                                col_index: (i%4).into(),
                                row_index: (i/4).into(),
                                ..default()
                            }}>
                                <BorderedBundle
                                    styles={KStyle{
                                        padding: Edge::all(Units::Pixels(7.5)).into(),
                                        ..default()
                                    }}
                                >
                                    <KImageBundle
                                        image={KImage(heart.clone())}
                                        styles={KStyle {
                                            width: StyleProp::Value(Units::Pixels(40.)),
                                            height: StyleProp::Value(Units::Pixels(40.)),
                                            left: Units::Stretch(1.0).into(),
                                            right: Units::Stretch(1.0).into(),
                                            top: Units::Stretch(1.0).into(),
                                            ..Default::default()
                                        }}
                                    />
                                    <TextWidgetBundle
                                        text={
                                            TextProps {
                                                content: "asa".into(),
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
                                    <TextWidgetBundle
                                        text={
                                            TextProps {
                                                content: "asa".into(),
                                                size: 15.0,
                                                alignment: Alignment::Middle.into(),
                                                ..Default::default()
                                            }
                                        }
                                        styles={
                                            KStyle {
                                                color: Color::ORANGE_RED.into(),
                                                padding_bottom: StyleProp::Value(Units::Pixels(5.)),
                                                bottom: Units::Stretch(1.0).into(),
                                                ..Default::default()
                                            }
                                        }
                                    />
                                </BorderedBundle>
                            </ElementBundle>
                        }
                    }}
                }
                <ElementBundle
                    styles={KStyle{
                    col_index: 1.into(),
                    row_index: 3.into(),
                    ..default()
                }}>
                    <TextWidgetBundle
                        text={
                            TextProps {
                                content: "<< Prev".into(),
                                size: 15.0,
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
                    col_index: 2.into(),
                    row_index: 3.into(),
                    ..default()
                }}>
                    <TextWidgetBundle
                        text={
                            TextProps {
                                content: "Next >>".into(),
                                size: 15.0,
                                ..Default::default()
                            }
                        }
                        styles={
                            KStyle {
                                color: Color::ORANGE_RED.into(),
                                padding_bottom: StyleProp::Value(Units::Pixels(5.)),
                                left: Units::Stretch(1.0).into(),
                                ..Default::default()
                            }
                        }
                    />
                </ElementBundle>
            </ElementBundle>
        </BorderedBundle>
    };

    true
}

// =========================== EXTERNALLY USED DEFINITIONS

#[derive(Resource, Default)]
pub struct MenuMenuInventoryResource {
    pub messages: Vec<String>,
}

// Our own version of widget_update that handles resource change events.
pub fn widget_update_with_resource<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((widget_context, entity, previous_entity)): In<(KayakWidgetContext, Entity, Entity)>,
    status_data: Res<MenuMenuInventoryResource>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    status_data.is_changed()
}

pub fn gui_plug_menumenuinventory(widget_context: &mut KayakRootContext, command: &mut Commands) {
    command.insert_resource(MenuMenuInventoryResource::default());

    widget_context.add_widget_data::<MenuMenuInventory, EmptyState>();
    widget_context.add_widget_system(
        MenuMenuInventory::default().get_name(),
        widget_update_with_resource::<MenuMenuInventory, EmptyState>,
        status_hud_renderer,
    );
}

#[derive(Bundle)]
pub struct MenuMenuInventoryBundle {
    pub menumenuinventory_hud: MenuMenuInventory,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
}

impl Default for MenuMenuInventoryBundle {
    fn default() -> Self {
        Self {
            menumenuinventory_hud: MenuMenuInventory::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            widget_name: MenuMenuInventory::default().get_name(),
        }
    }
}
