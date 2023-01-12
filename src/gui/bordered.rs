use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

// =========================== INTERNALLY ABSTRACTED DEFINITIONS

#[derive(Component, Default, PartialEq, Clone)]
pub struct Bordered;

impl Widget for Bordered {}

fn sample_widget_renderer(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut query: Query<(&KStyle, &mut ComputedStyles, &KChildren)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) -> bool {
    if let Ok((window_style, mut computed_styles, window_children)) = query.get_mut(entity) {
        let cell = asset_server.load("image_gui/boxes/mini_box_01.png");

        let parent_id = Some(entity);
        rsx! {
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: cell,
                    border: Edge::all(16.0),
                }}
                styles={window_style.clone()}

            >
                // <ClipBundle>
                //     <ScrollContextProviderBundle>
                //         <ScrollBoxBundle
                //             scroll_box_props={ScrollBoxProps {
                //                 always_show_scrollbar: false,
                //                 ..Default::default()
                //             }}
                //             children={window_children.clone()}
                //         />
                //     </ScrollContextProviderBundle>
                // </ClipBundle>
                <ClipBundle
                    children={window_children.clone()}
                />
                // <ScrollContextProviderBundle>
                //     <ScrollBoxBundle
                //         scroll_box_props={ScrollBoxProps {
                //             always_show_scrollbar: false,
                //             ..Default::default()
                //         }}
                //         children={window_children.clone()}
                //     />
                // </ScrollContextProviderBundle>
            </NinePatchBundle>
        };
    }

    true
}

// =========================== EXTERNALLY USED DEFINITIONS

pub fn gui_plug_bordered_widget(widget_context: &mut KayakRootContext) {
    widget_context.add_widget_data::<Bordered, EmptyState>();
    widget_context.add_widget_system(
        Bordered::default().get_name(),
        widget_update::<Bordered, EmptyState>,
        sample_widget_renderer,
    );
}

#[derive(Bundle)]
pub struct BorderedBundle {
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
    pub children: KChildren,
}

impl Default for BorderedBundle {
    fn default() -> Self {
        Self {
            styles: KStyle {
                padding: Edge::axis(Units::Pixels(8.0), Units::Pixels(10.0)).into(),
                ..KStyle::default()
            },
            computed_styles: ComputedStyles::default(),
            widget_name: Bordered::default().get_name(),
            children: Default::default(),
        }
    }
}
