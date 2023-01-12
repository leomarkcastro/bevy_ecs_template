use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::gui::crop_image::CropImageBundle;

use super::{
    bordered::{gui_plug_bordered_widget, BorderedBundle},
    crop_image::gui_plug_crop_image,
};

// =========================== INTERNALLY ABSTRACTED DEFINITIONS

#[derive(Component, Default, PartialEq, Clone)]
pub struct MenuMenuList;

impl Widget for MenuMenuList {}

fn status_hud_renderer(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    status_data: Res<MenuMenuListResource>,
    asset_server: Res<AssetServer>,
) -> bool {
    let parent_id = Some(entity);
    let state = status_data;
    // let heart = asset_server.load("image_gui/images/icon_mono_health.png");
    rsx! {
        <BorderedBundle
            styles={KStyle{
                padding: Edge::all(Units::Pixels(7.5)).into(),
                ..default()
            }}
        >
            <ElementBundle
                styles={KStyle{
                    layout_type: LayoutType::Row.into(),
                    col_between: Units::Pixels(20.0).into(),
                    top: Units::Stretch(0.30).into(),
                    ..default()
                }}
            >
                <TextWidgetBundle
                    text={
                        TextProps {
                            content: "Inventory".into(),
                            size: 16.0,
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
                            content: "Crafting".into(),
                            size: 16.0,
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
                            content: "Settings".into(),
                            size: 16.0,
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
            </ElementBundle>
        </BorderedBundle>
    };

    true
}

// =========================== EXTERNALLY USED DEFINITIONS

#[derive(Resource, Default)]
pub struct MenuMenuListResource {
    pub messages: Vec<String>,
}

// Our own version of widget_update that handles resource change events.
pub fn widget_update_with_resource<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((widget_context, entity, previous_entity)): In<(KayakWidgetContext, Entity, Entity)>,
    status_data: Res<MenuMenuListResource>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    status_data.is_changed()
}

pub fn gui_plug_menumenulist(widget_context: &mut KayakRootContext, command: &mut Commands) {
    command.insert_resource(MenuMenuListResource::default());

    widget_context.add_widget_data::<MenuMenuList, EmptyState>();
    widget_context.add_widget_system(
        MenuMenuList::default().get_name(),
        widget_update_with_resource::<MenuMenuList, EmptyState>,
        status_hud_renderer,
    );
}

#[derive(Bundle)]
pub struct MenuMenuListBundle {
    pub menumenulist_hud: MenuMenuList,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
}

impl Default for MenuMenuListBundle {
    fn default() -> Self {
        Self {
            menumenulist_hud: MenuMenuList::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            widget_name: MenuMenuList::default().get_name(),
        }
    }
}
