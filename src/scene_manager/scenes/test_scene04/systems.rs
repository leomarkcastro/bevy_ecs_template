// To describe how the Scene04 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    entity_factory::{
        entities::playerv2::entities::Playerv2Entity,
        factory::data::{GameEntity, SpawnEntityEvent},
    },
    game_modules::{
        camera::components::{CameraFollowable, CameraMode, CameraResource},
        controllable::components::ControllableResource,
        shaders::simple_point_light::components::CoolMaterialUniformInput,
        time_system::systems::CurrentWorldTimeGlobal,
        timers::components::{HalfMilliSecondTimer, OneSecondTimer},
    },
    gui::{
        bordered::{gui_plug_bordered_widget, BorderedBundle},
        crop_image::{gui_plug_crop_image, CropImageBundle},
        sample_with_state::{gui_plug_sample_widget, SampleWidgetBundle, SampleWidgetState},
    },
    scene_manager::manager::{
        entities::{SpawnAt, World01},
        scene_list::GameScene,
        utils::despawn_screen,
    },
};

pub struct Scene04Plugin;

impl Plugin for Scene04Plugin {
    fn build(&self, app: &mut App) {
        app // When entering the state, spawn everything needed for this screen
            .add_system_set(
                SystemSet::on_enter(GameScene::Scene04).with_system(scene04_init_system),
            )
            .add_system_set(SystemSet::on_update(GameScene::Scene04).with_system(scene04_update))
            .add_system_set(
                SystemSet::on_update(GameScene::Scene04)
                    .with_system(scene04_follow_first_player_system),
            )
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(
                SystemSet::on_exit(GameScene::Scene04).with_system(despawn_screen::<World01>),
            );
    }
}

fn scene04_init_system(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("kayak_fonts/dos_tall.kayak_font"));
    let image = asset_server.load("image_gui/images/gun_pistol.png");

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    gui_plug_bordered_widget(&mut widget_context);
    gui_plug_sample_widget(&mut widget_context);
    gui_plug_crop_image(&mut widget_context);
    let parent_id = None;

    // The rsx! macro expects a parent_id, a widget_context from the user.
    // It also expects `Commands` from bevy.
    // This can be a little weird at first.
    // See the rsx! docs for more info!
    let lorem_ipsum = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras sed tellus neque. Proin tempus ligula a mi molestie aliquam. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nullam venenatis consequat ultricies. Sed ac orci purus. Nullam velit nisl, dapibus vel mauris id, dignissim elementum sapien. Vestibulum faucibus sapien ut erat bibendum, id lobortis nisi luctus. Mauris feugiat at lectus at pretium. Pellentesque vitae finibus ante. Nulla non ex neque. Cras varius, lorem facilisis consequat blandit, lorem mauris mollis massa, eget consectetur magna sem vel enim. Nam aliquam risus pulvinar, volutpat leo eget, eleifend urna. Suspendisse in magna sed ligula vehicula volutpat non vitae augue. Phasellus aliquam viverra consequat. Nam rhoncus molestie purus, sed laoreet neque imperdiet eget. Sed egestas metus eget sodales congue.
                                    
 Sed vel ante placerat, posuere lacus sit amet, tempus enim. Cras ullamcorper ex vitae metus consequat, a blandit leo semper. Nunc lacinia porta massa, a tempus leo laoreet nec. Sed vel metus tincidunt, scelerisque ex sit amet, lacinia dui. In sollicitudin pulvinar odio vitae hendrerit. Maecenas mollis tempor egestas. Nulla facilisi. Praesent nisi turpis, accumsan eu lobortis vestibulum, ultrices id nibh. Suspendisse sed dui porta, mollis elit sed, ornare sem. Cras molestie est libero, quis faucibus leo semper at.
                                    
 Nulla vel nisl rutrum, fringilla elit non, mollis odio. Donec convallis arcu neque, eget venenatis sem mattis nec. Nulla facilisi. Phasellus risus elit, vehicula sit amet risus et, sodales ultrices est. Quisque vulputate felis orci, non tristique leo faucibus in. Duis quis velit urna. Sed rhoncus dolor vel commodo aliquet. In sed tempor quam. Nunc non tempus ipsum. Praesent mi lacus, vehicula eu dolor eu, condimentum venenatis diam. In tristique ligula a ligula dictum, eu dictum lacus consectetur. Proin elementum egestas pharetra. Nunc suscipit dui ac nisl maximus, id congue velit volutpat. Etiam condimentum, mauris ac sodales tristique, est augue accumsan elit, ut luctus est mi ut urna. Mauris commodo, tortor eget gravida lacinia, leo est imperdiet arcu, a ullamcorper dui sapien eget erat.
                                
 Vivamus pulvinar dui et elit volutpat hendrerit. Praesent luctus dolor ut rutrum finibus. Fusce ut odio ultrices, laoreet est at, condimentum turpis. Morbi at ultricies nibh. Mauris tempus imperdiet porta. Proin sit amet tincidunt eros. Quisque rutrum lacus ac est vehicula dictum. Pellentesque nec augue mi.
                                
 Vestibulum rutrum imperdiet nisl, et consequat massa porttitor vel. Ut velit justo, vehicula a nulla eu, auctor eleifend metus. Ut egestas malesuada metus, sit amet pretium nunc commodo ac. Pellentesque gravida, nisl in faucibus volutpat, libero turpis mattis orci, vitae tincidunt ligula ligula ut tortor. Maecenas vehicula lobortis odio in molestie. Curabitur dictum elit sed arcu dictum, ut semper nunc cursus. Donec semper felis non nisl tincidunt elementum.
    "#.to_string();

    rsx! {
        <KayakAppBundle>
            <ElementBundle
                styles={KStyle {
                    row_between: StyleProp::Value(Units::Pixels(4.0)),
                    ..Default::default()
                }}
            >
                <BorderedBundle>
                    <TextWidgetBundle
                        styles={KStyle {
                            color: StyleProp::Value(Color::ORANGE_RED),
                            ..Default::default()
                        }}
                        text={TextProps {
                            content: "Hello World".into(),
                            size: 40.0,
                            ..Default::default()
                        }}
                    />
                    <TextWidgetBundle
                        styles={KStyle {
                            color: StyleProp::Value(Color::PINK),
                            ..Default::default()
                        }}
                        text={TextProps {
                            content: "Kayak UI".into(),
                            size: 40.0,
                            ..Default::default()
                        }}
                    />
                    <TextWidgetBundle
                        styles={KStyle {
                            color: StyleProp::Value(Color::ORANGE_RED),
                            ..Default::default()
                        }}
                        text={TextProps {
                            content: "Hello World".into(),
                            size: 40.0,
                            ..Default::default()
                        }}
                    />
                    <TextWidgetBundle
                        styles={KStyle {
                            color: StyleProp::Value(Color::PINK),
                            ..Default::default()
                        }}
                        text={TextProps {
                            content: "Kayak UI".into(),
                            size: 40.0,
                            ..Default::default()
                        }}
                    />
                    <KImageBundle
                        image={KImage(image)}
                        styles={KStyle {
                            width: StyleProp::Value(Units::Pixels(79.0)),
                            height: StyleProp::Value(Units::Pixels(46.0)),
                            ..Default::default()
                        }}
                    />
                </BorderedBundle>
                <BorderedBundle>
                    <TextWidgetBundle
                        styles={KStyle {
                            color: StyleProp::Value(Color::ORANGE_RED),
                            ..Default::default()
                        }}
                        text={TextProps {
                            content: lorem_ipsum.into(),
                            size: 12.0,
                            ..Default::default()
                        }}
                    />
                </BorderedBundle>
                <BorderedBundle>
                    <TextWidgetBundle
                        styles={KStyle {
                            color: StyleProp::Value(Color::ORANGE_RED),
                            ..Default::default()
                        }}
                        text={TextProps {
                            content: "Hello World".into(),
                            size: 40.0,
                            ..Default::default()
                        }}
                    />

                    <CropImageBundle styles= {
                        KStyle {
                            width: StyleProp::Value(Units::Pixels(40.0)),
                            height: StyleProp::Value(Units::Pixels(10.0)),
                            ..Default::default()
                        }
                    } />
                    <KButtonBundle
                        button={KButton {
                            text: "Show Window".into(),
                        }}
                        on_event={OnEvent::new(
                            move |In((event_dispatcher_context, _, mut event, _entity)): In<(EventDispatcherContext, WidgetState, Event, Entity)>| {
                                event.prevent_default();
                                event.stop_propagation();
                                match event.event_type {
                                    EventType::Click(..) => {
                                        println!("Show Window Clicked");
                                    }
                                    _ => {}
                                }
                                (event_dispatcher_context, event)
                            },
                        )}
                    />
                    <KButtonBundle
                        button={KButton {
                            text: "Show Window".into(),
                        }}
                        on_event={OnEvent::new(
                            move |In((event_dispatcher_context, _, mut event, _entity)): In<(EventDispatcherContext, WidgetState, Event, Entity)>| {
                                event.prevent_default();
                                event.stop_propagation();
                                match event.event_type {
                                    EventType::Click(..) => {
                                        println!("Show Window Clicked");
                                    }
                                    _ => {}
                                }
                                (event_dispatcher_context, event)
                            },
                        )}
                    />
                </BorderedBundle>

                <CropImageBundle styles= {
                        KStyle {
                            width: StyleProp::Value(Units::Pixels(79.0)),
                            height: StyleProp::Value(Units::Pixels(10.0)),
                            ..Default::default()
                        }
                    } />
                <CropImageBundle styles= {
                        KStyle {
                            width: StyleProp::Value(Units::Pixels(79.0)),
                            height: StyleProp::Value(Units::Pixels(10.0)),
                            ..Default::default()
                        }
                    } />

                <WindowContextProviderBundle>
                    <WindowBundle
                        window={KWindow {
                            title: "State Example Window".into(),
                            draggable: true,
                            initial_position: Vec2::new(10.0, 10.0),
                            size: Vec2::new(300.0, 250.0),
                            ..KWindow::default()
                        }}
                    >
                        <SampleWidgetBundle />
                    </WindowBundle>
                </WindowContextProviderBundle>
            </ElementBundle>
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}

fn scene04_update(
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    controllable_component: Res<ControllableResource>,
) {
    if (controllable_component.btn_c.pressed) {
        spawn_entity_events.send(SpawnEntityEvent {
            entity: GameEntity::PlayerV2,
            position: Some(Vec3::from([0.0, 0.0, 20.0])),
            ..Default::default()
        });
    }
}

fn scene04_follow_first_player_system(
    mut camera_resource: ResMut<CameraResource>,
    mut colordata_query: Query<
        (&mut CoolMaterialUniformInput, &Transform),
        Without<Playerv2Entity>,
    >,
    player_query: Query<(&CameraFollowable, &mut Transform), With<Playerv2Entity>>,
) {
    if let Ok((&pl_followable, mut pl_transform)) = player_query.get_single() {
        if let CameraMode::AtAssetFace {
            target_asset_id,
            distance,
        } = camera_resource.mode
        {
            return;
        }
        // println!("CameraMode::AtAsset");
        let followable_id = pl_followable.id;
        camera_resource.mode = CameraMode::AtAssetFace {
            target_asset_id: followable_id,
            distance: 30.0,
        };
        camera_resource.speed = 0.04;
    }
}
