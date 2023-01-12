use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::gui::crop_image::CropImageBundle;

use super::{
    bordered::{gui_plug_bordered_widget, BorderedBundle},
    crop_image::gui_plug_crop_image,
};

// =========================== INTERNALLY ABSTRACTED DEFINITIONS

#[derive(Component, Default, PartialEq, Clone)]
pub struct StatusHud;

impl Widget for StatusHud {}

fn status_hud_renderer(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    status_data: Res<StatusHUDResource>,
    asset_server: Res<AssetServer>,
) -> bool {
    let parent_id = Some(entity);
    let state = status_data;

    let heart = asset_server.load("image_gui/images/icon_mono_health.png");
    let armor = asset_server.load("image_gui/images/icon_mono_armor.png");
    let weight = asset_server.load("image_gui/images/icon_mono_weight.png");
    let ammo = asset_server.load("image_gui/images/icon_ammo.png");
    let pistol = asset_server.load("image_gui/images/gun_pistol.png");
    rsx! {
            <BorderedBundle
                styles={KStyle {
                    width: Units::Pixels(180.0).into(),
                    height: Units::Pixels(100.0).into(),
                    // row_index: 2.into(),
                    // col_index: 1.into(),
                    ..Default::default()
                }}
            >
                <ElementBundle
                    styles={KStyle{
                        layout_type: LayoutType::Row.into(),
                        padding: Edge::axis(Units::Pixels(5.0), Units::Pixels(5.0)).into(),
                        ..default()
                    }}
                >
                    <ElementBundle
                        styles={KStyle{
                            layout_type: LayoutType::Column.into(),
                            row_between: Units::Pixels(1.0).into(),
                            padding: Edge::axis(Units::Pixels(12.5), Units::Pixels(7.5)).into(),
                            ..default()
                        }}
                    >
                        <KImageBundle
                            image={KImage(pistol)}
                            styles={KStyle {
                                width: StyleProp::Value(Units::Pixels(60.)),
                                height: StyleProp::Value(Units::Pixels(45.)),
                                padding_top: StyleProp::Value(Units::Pixels(25.)),
                                ..Default::default()
                            }}
                        />
                        <ElementBundle
                            styles={KStyle{
                                layout_type: LayoutType::Row.into(),
                                col_between: Units::Pixels(5.0).into(),
                                ..default()
                            }}
                        >
                            <KImageBundle
                                image={KImage(ammo)}
                                styles={KStyle {
                                    width: StyleProp::Value(Units::Pixels(10.)),
                                    height: StyleProp::Value(Units::Pixels(10.)),
                                    ..Default::default()
                                }}
                            />
                            <CropImageBundle styles= {
                                KStyle {
                                    width: StyleProp::Value(Units::Pixels(40.0)),
                                    height: StyleProp::Value(Units::Pixels(10.0)),
                                    line_height: 10.0.into(),
                                    ..Default::default()
                                }
                            } />
                        </ElementBundle>
                    </ElementBundle>
                    <ElementBundle
                        styles={KStyle{
                            layout_type: LayoutType::Column.into(),
                            row_between: Units::Pixels(1.0).into(),
                            padding: Edge::axis(Units::Pixels(2.5), Units::Pixels(0.5)).into(),
                            ..default()
                        }}
                    >
                        <TextWidgetBundle
                            text={
                                TextProps {
                                    content: format!("{} : {}", state.hour, state.minute),
                                    size: 20.0,
                                    ..Default::default()
                                }
                            }
                            styles={
                                KStyle {
                                    color: Color::rgb(0.5, 0.5, 0.5).into(),
                                    ..Default::default()
                                }
                            }
                        />
                        <ElementBundle
                            styles={KStyle{
                                layout_type: LayoutType::Row.into(),
                                col_between: Units::Pixels(5.0).into(),
                                ..default()
                            }}
                        >
                            <KImageBundle
                                image={KImage(heart)}
                                styles={KStyle {
                                    width: StyleProp::Value(Units::Pixels(10.)),
                                    height: StyleProp::Value(Units::Pixels(10.)),
                                    ..Default::default()
                                }}
                            />
                            <CropImageBundle styles= {
                                KStyle {
                                    width: StyleProp::Value(Units::Pixels(60.0)),
                                    height: StyleProp::Value(Units::Pixels(10.0)),
                                    line_height: 10.0.into(),
                                    ..Default::default()
                                }
                            } />
                        </ElementBundle>
                        <ElementBundle
                            styles={KStyle{
                                layout_type: LayoutType::Row.into(),
                                col_between: Units::Pixels(5.0).into(),
                                ..default()
                            }}
                        >
                            <KImageBundle
                                image={KImage(armor)}
                                styles={KStyle {
                                    width: StyleProp::Value(Units::Pixels(10.)),
                                    height: StyleProp::Value(Units::Pixels(10.)),
                                    ..Default::default()
                                }}
                            />
                            <CropImageBundle styles= {
                                KStyle {
                                    width: StyleProp::Value(Units::Pixels(60.0)),
                                    height: StyleProp::Value(Units::Pixels(10.0)),
                                    line_height: 10.0.into(),
                                    ..Default::default()
                                }
                            } />
                        </ElementBundle>
                        <ElementBundle
                            styles={KStyle{
                                layout_type: LayoutType::Row.into(),
                                col_between: Units::Pixels(5.0).into(),
                                ..default()
                            }}
                        >
                            <KImageBundle
                                image={KImage(weight)}
                                styles={KStyle {
                                    width: StyleProp::Value(Units::Pixels(10.)),
                                    height: StyleProp::Value(Units::Pixels(10.)),
                                    ..Default::default()
                                }}
                            />
                            <CropImageBundle styles= {
                                KStyle {
                                    width: StyleProp::Value(Units::Pixels(60.0)),
                                    height: StyleProp::Value(Units::Pixels(10.0)),
                                    line_height: 10.0.into(),
                                    ..Default::default()
                                }
                            } />
                        </ElementBundle>
                    </ElementBundle>
                </ElementBundle>
            </BorderedBundle>
    };

    true
}

// =========================== EXTERNALLY USED DEFINITIONS

#[derive(Resource, Default)]
pub struct StatusHUDResource {
    pub hour: u32,
    pub minute: u32,
    pub health: f32,
    pub armor: f32,
    pub hunger: f32,
    pub bag: f32,
    pub weapon: String,
    pub ammo: u32,
}

// Our own version of widget_update that handles resource change events.
pub fn widget_update_with_resource<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((widget_context, entity, previous_entity)): In<(KayakWidgetContext, Entity, Entity)>,
    status_data: Res<StatusHUDResource>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    status_data.is_changed()
}

pub fn gui_plug_status_hud(widget_context: &mut KayakRootContext, command: &mut Commands) {
    command.insert_resource(StatusHUDResource::default());

    widget_context.add_widget_data::<StatusHud, EmptyState>();
    widget_context.add_widget_system(
        StatusHud::default().get_name(),
        widget_update_with_resource::<StatusHud, EmptyState>,
        status_hud_renderer,
    );

    gui_plug_bordered_widget(widget_context);
    gui_plug_crop_image(widget_context);
}

#[derive(Bundle)]
pub struct StatusHudBundle {
    pub status_hud: StatusHud,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
}

impl Default for StatusHudBundle {
    fn default() -> Self {
        Self {
            status_hud: StatusHud::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            widget_name: StatusHud::default().get_name(),
        }
    }
}
