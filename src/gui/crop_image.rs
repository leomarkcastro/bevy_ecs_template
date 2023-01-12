use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

// =========================== INTERNALLY ABSTRACTED DEFINITIONS

#[derive(Component, Default, PartialEq, Clone)]
pub struct CropImage;

impl Widget for CropImage {}

#[derive(Component, PartialEq, Clone)]
pub struct CropImageState {
    pub percentage: f32,
    box_width: f32,
    box_height: f32,
}

impl Default for CropImageState {
    fn default() -> Self {
        CropImageState {
            percentage: 0.95,
            box_width: 0.0,
            box_height: 0.0,
        }
    }
}

fn crop_image_renderer(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    mut query: Query<(&KStyle, &mut ComputedStyles, &mut OnLayout)>,
    mut query_val: Query<(&mut CropImageState)>,
    asset_server: Res<AssetServer>,
) -> bool {
    let state_entity = widget_context.use_state(&mut commands, entity, CropImageState::default());
    let image_back = asset_server.load("image_gui/loading/bar01_back.png");
    let image_front = asset_server.load("image_gui/loading/bar01_front.png");

    if let Ok((window_style, mut computed_styles, mut on_layout)) = query.get_mut(entity) {
        let def_state = CropImageState::default();
        let current_state = query_val.get(state_entity).unwrap_or(&def_state);

        // === OnLayout === //
        *on_layout = OnLayout::new(
            move |In((event, _entity)): In<(LayoutEvent, Entity)>,
                  mut query_crop: Query<&mut CropImageState>| {
                if event
                    .flags
                    .intersects(GeometryChanged::WIDTH_CHANGED | GeometryChanged::HEIGHT_CHANGED)
                {
                    if let Ok(mut scroll) = query_crop.get_mut(state_entity.clone()) {
                        scroll.box_width = event.layout.width;
                        scroll.box_height = event.layout.height;
                    }
                }

                event
            },
        );

        // === Styles === //
        // *computed_styles = KStyle::default()
        //     .with_style(KStyle {
        //         width: Units::Auto.into(),
        //         height: Units::Auto.into(),
        //         ..Default::default()
        //     })
        //     .with_style(window_style)
        //     .into();

        let parent_id = Some(entity);

        // let width = &computed_styles.0.width.resolve().value_or(0.0, 0.0);
        // let height = &computed_styles.0.height.resolve().value_or(0.0, 0.0);

        let width = window_style
            .width
            .resolve()
            .value_or(0.0, current_state.box_width);
        let height = window_style
            .height
            .resolve()
            .value_or(0.0, current_state.box_height);

        let margin = 1. / 10. * height;
        let bar_width = width - margin * 2.;
        let bar_height = height - margin * 2.;

        // println!("[{:?} {:?}]", width, height);

        rsx! {
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: image_back,
                    border: Edge::all(2.0),
                }}
                styles={window_style.clone()}
            >
                <ClipBundle
                    styles={KStyle {
                        left: StyleProp::Value(Units::Pixels(1.)),
                        top: StyleProp::Value(Units::Pixels(margin)),
                        width: StyleProp::Value(Units::Pixels(bar_width * current_state.percentage)),
                        ..Default::default()
                    }}
                >
                    <KImageBundle
                        image={KImage(image_front)}
                        styles={KStyle {
                            width: StyleProp::Value(Units::Pixels(bar_width)),
                            height: StyleProp::Value(Units::Pixels(bar_height)),
                            ..Default::default()
                        }}
                    />
                </ClipBundle>
            </NinePatchBundle>
        };
    }

    true
}

// =========================== EXTERNALLY USED DEFINITIONS

pub fn gui_plug_crop_image(widget_context: &mut KayakRootContext) {
    widget_context.add_widget_data::<CropImage, CropImageState>();
    widget_context.add_widget_system(
        CropImage::default().get_name(),
        widget_update::<CropImage, CropImageState>,
        crop_image_renderer,
    );
}

#[derive(Bundle)]
pub struct CropImageBundle {
    pub cropped_widget: CropImage,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
    pub on_layout: OnLayout,
}

impl Default for CropImageBundle {
    fn default() -> Self {
        Self {
            cropped_widget: CropImage::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            widget_name: CropImage::default().get_name(),
            on_layout: Default::default(),
        }
    }
}
