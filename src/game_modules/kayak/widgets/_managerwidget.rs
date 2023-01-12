use bevy::{prelude::*, render::camera::ScalingMode};
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    game_modules::camera::systems::{PROJECTION_SIZE, RESOLUTION},
    gui::{
        dialogue::{gui_plug_dialogue, DialogueBundle},
        menu_craft::{gui_plug_menucraft, MenuCraftBundle},
        menu_menuinventory::{gui_plug_menumenuinventory, MenuMenuInventoryBundle},
        menu_menulist::{gui_plug_menumenulist, MenuMenuListBundle, MenuMenuListResource},
        menu_overview::{gui_plug_menuoverview, MenuOverviewBundle},
        notifications::{gui_plug_notifications, NotificationsBundle},
        status_hud::{gui_plug_status_hud, StatusHudBundle},
        tasks::{gui_plug_tasks, TasksBundle},
    },
};

#[derive(Component, Default, PartialEq, Clone)]
pub struct ManagerWidget;

impl Widget for ManagerWidget {}

#[derive(Bundle)]
struct ManagerWidgetBundle {
    pub widget: ManagerWidget,
    pub styles: KStyle,
    pub widget_name: WidgetName,
}

impl Default for ManagerWidgetBundle {
    fn default() -> Self {
        Self {
            widget: ManagerWidget::default(),
            styles: KStyle::default(),
            widget_name: ManagerWidget::default().get_name(),
        }
    }
}

// Our own version of widget_update that handles resource change events.
pub fn widget_update_on_resource_update<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((widget_context, entity, previous_entity)): In<(KayakWidgetContext, Entity, Entity)>,
    widget_manager: Res<WidgetManagerResource>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    widget_manager.is_changed()
}

// #=============================== Declare components here

#[derive(Resource, Default)]
pub struct WidgetManagerResource {
    pub status_hud: bool,
    pub notifications: bool,
    pub quest: bool,
    pub menu_menulist: bool,
    pub menu_menuinventory: bool,
    pub menu_overview: bool,
    pub menu_craft: bool,
}

fn manager_widget_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    widget_manager: Res<WidgetManagerResource>,
) -> bool {
    let parent_id = Some(entity);
    let state = widget_manager;
    rsx! {
        <ElementBundle styles={
            KStyle {
                padding: Edge::all(Units::Pixels(6.0)).into(),
                layout_type: LayoutType::Grid.into(),
                grid_rows: vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0)].into(),
                grid_cols: vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0)].into(),
                row_between: Units::Pixels(6.0).into(),
                col_between: Units::Pixels(6.0).into(),
                ..Default::default()
            }
        }>
            <BackgroundBundle styles={KStyle {
              row_index: 0.into(),
              col_index: 0.into(),
              col_span: 2.into(),
              row_span: 2.into(),
              ..Default::default()
            }}>
              {if state.status_hud {
                  constructor! {
                      <StatusHudBundle />
                  }
              }}
            </BackgroundBundle>
            <BackgroundBundle styles={KStyle {
              row_index: 2.into(),
              col_index: 0.into(),
              col_span: 2.into(),
              row_span: 3.into(),
              ..Default::default()
            }}>
              {if state.notifications {
                  constructor! {
                      <NotificationsBundle />
                  }
              }}
            </BackgroundBundle>
            <BackgroundBundle styles={KStyle {
              row_index: 0.into(),
              col_index: 10.into(),
              col_span: 2.into(),
              row_span: 2.into(),
              ..Default::default()
            }}>
              {if state.notifications {
                  constructor! {
                      <DialogueBundle />
                  }
              }}
            </BackgroundBundle>
            <BackgroundBundle styles={KStyle {
              row_index: 1.into(),
              col_index: 1.into(),
              col_span: 10.into(),
              row_span: 1.into(),
              ..Default::default()
            }}>
              {if state.menu_menulist {
                  constructor! {
                      <MenuMenuListBundle />
                  }
              }}
            </BackgroundBundle>
            <BackgroundBundle styles={KStyle {
              row_index: 2.into(),
              col_index: 4.into(),
              col_span: 7.into(),
              row_span: 8.into(),
              ..Default::default()
            }}>
              {if state.menu_menuinventory {
                  constructor! {
                      <MenuMenuInventoryBundle />
                  }
              }}
            </BackgroundBundle>
            <BackgroundBundle styles={KStyle {
              row_index: 2.into(),
              col_index: 1.into(),
              col_span: 3.into(),
              row_span: 8.into(),
              ..Default::default()
            }}>
              {if state.menu_overview {
                  constructor! {
                      <MenuOverviewBundle />
                  }
              }}
            </BackgroundBundle>
            <BackgroundBundle styles={KStyle {
              row_index: 2.into(),
              col_index: 1.into(),
              col_span: 3.into(),
              row_span: 8.into(),
              ..Default::default()
            }}>
              {if state.menu_craft {
                  constructor! {
                      <MenuCraftBundle />
                  }
              }}
            </BackgroundBundle>
        </ElementBundle>
    };

    true
}

pub fn gui_inject_manager_widget(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("kayak_fonts/dos_tall.kayak_font"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);

    let parent_id = None;
    widget_context.add_widget_data::<ManagerWidget, EmptyState>();
    widget_context.add_widget_system(
        ManagerWidget::default().get_name(),
        widget_update_on_resource_update::<ManagerWidget, EmptyState>,
        manager_widget_render,
    );

    gui_plug_status_hud(&mut widget_context, &mut commands);
    gui_plug_notifications(&mut widget_context, &mut commands);
    gui_plug_dialogue(&mut widget_context, &mut commands);
    gui_plug_tasks(&mut widget_context, &mut commands);
    gui_plug_menumenulist(&mut widget_context, &mut commands);
    gui_plug_menumenuinventory(&mut widget_context, &mut commands);
    gui_plug_menuoverview(&mut widget_context, &mut commands);
    gui_plug_menucraft(&mut widget_context, &mut commands);

    rsx! {
        <KayakAppBundle>
            <ManagerWidgetBundle />
        </KayakAppBundle>
    };

    commands.insert_resource(WidgetManagerResource {
        status_hud: true,
        // notifications: true,
        // quest: true,
        // menu_menulist: true,
        // menu_craft: true,
        // menu_menuinventory: true,
        ..Default::default()
    });
    let mut ui_camera = UICameraBundle::new(widget_context);

    // ui_camera.orthographic_projection.right = PROJECTION_SIZE * RESOLUTION;
    // ui_camera.orthographic_projection.left = -PROJECTION_SIZE * RESOLUTION;

    // ui_camera.orthographic_projection.top = PROJECTION_SIZE;
    // ui_camera.orthographic_projection.bottom = -PROJECTION_SIZE;

    // ui_camera.orthographic_projection.scaling_mode = ScalingMode::None;
    // ui_camera.orthographic_projection.scale = 2.0;
    // println!("{:?}", ui_camera.orthographic_projection);
    commands.spawn(ui_camera);
}
