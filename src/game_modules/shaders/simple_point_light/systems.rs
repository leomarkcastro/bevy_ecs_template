// To describe how the SimplePointLight component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{
    math::Vec3Swizzles,
    prelude::{shape::Quad, *},
    render::{
        render_resource::{encase, OwnedBindingResource},
        renderer::RenderQueue,
        Extract, RenderApp, RenderStage,
    },
    sprite::{Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle, RenderMaterials2d},
};
use bevy_inspector_egui::RegisterInspectable;

use crate::{
    game_modules::{
        camera::systems::{PROJECTION_SIZE, RESOLUTION},
        time_system::systems::CurrentWorldTimeGlobal,
    },
    utils::value_range_map::map_range_fromxy_toxy,
};

use super::components::{CoolMaterial, CoolMaterialUniformBuffer, CoolMaterialUniformInput};

pub struct SimplePointLightPlugin;

impl Plugin for SimplePointLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(simple_point_light_init_system)
            .add_plugin(Material2dPlugin::<CoolMaterial>::default())
            .register_inspectable::<CoolMaterialUniformInput>()
            .add_system(make_light_frame_follow_camera_system)
            .add_system(day_night_cycle_with_timesystem_system)
            .add_system(adjust_colordata_via_kb);

        // Add all render world systems/resources
        app.sub_app_mut(RenderApp)
            .add_system_to_stage(RenderStage::Extract, simple_point_light_extract_system)
            .add_system_to_stage(RenderStage::Prepare, simple_point_light_prepare_system);
    }
}

const OVERLAY_SIZE: [f32; 2] = [PROJECTION_SIZE * RESOLUTION * 2., PROJECTION_SIZE * 2.];

fn simple_point_light_init_system(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<CoolMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: mesh_assets
                .add(Mesh::from(shape::Quad::from(Quad {
                    size: Vec2::new(OVERLAY_SIZE[0], OVERLAY_SIZE[1]) * 1.5,
                    ..Default::default()
                })))
                .into(),
            material: my_material_assets.add(CoolMaterial {
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 700.0),
            ..default()
        })
        .insert(CoolMaterialUniformInput {
            // color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            ..Default::default()
        });
}

fn simple_point_light_extract_system(
    mut commands: Commands,
    materialinput_query: Extract<Query<(Entity, &CoolMaterialUniformInput, &Handle<CoolMaterial>)>>,
) {
    for (entity, material_input, handle) in materialinput_query.iter() {
        commands
            .get_or_spawn(entity)
            .insert(*material_input)
            .insert(handle.clone());
    }
}

fn simple_point_light_prepare_system(
    materials: Res<RenderMaterials2d<CoolMaterial>>,
    health_query: Query<(&CoolMaterialUniformInput, &Handle<CoolMaterial>)>,
    render_queue: Res<RenderQueue>,
) {
    for (material_input, handle) in health_query.iter() {
        if let Some(material) = materials.get(handle) {
            let binding = &material.bindings[0];
            if let OwnedBindingResource::Buffer(cur_buffer) = binding {
                let mut buffer = encase::UniformBuffer::new(Vec::new());
                buffer
                    .write(&CoolMaterialUniformBuffer {
                        color: material_input.color,
                        position: material_input.position,
                    })
                    .unwrap();
                render_queue.write_buffer(cur_buffer, 0, buffer.as_ref());
            }
        }
    }
}

const TIME_SKIP: f32 = 1. / 60.;
const SPEED: f32 = 100.0;

fn make_light_frame_follow_camera_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut colordata_query: Query<(&mut CoolMaterialUniformInput, &mut Transform), Without<Camera2d>>,
) {
    let camera_xy = camera_query.single().translation.xyy().xy();
    let (_, mut position) = colordata_query.single_mut();
    position.translation = Vec3::new(camera_xy.x, camera_xy.y, position.translation.z);
}

fn day_night_cycle_with_timesystem_system(
    time_tick: Res<CurrentWorldTimeGlobal>,
    mut colordata_query: Query<(&mut CoolMaterialUniformInput, &mut Transform), Without<Camera2d>>,
) {
    // from 8am to 1pm, darkness_level = 0.0 -> 0.0
    // from 1pm to 4pm, darkness_level = 0.0 -> 0.7
    // from 4pm to 8pm, darkness_level = 0.7 -> 0.95
    // from 8pm to 2am, darkness_level = 0.95 -> 1.0
    // from 2am to 4am, darkness_level = 1.0 -> 0.95
    // from 8pm to 4am, darkness_level = 0.95 -> 0.9
    // from 4am to 8am, darkness_level = 0.9 -> 0.0

    let mut darkness_level = 0.0;
    let hour = time_tick.hours;

    if (hour >= 8 && hour < 13) {
        darkness_level = 0.0;
    } else if (hour >= 13 && hour < 16) {
        darkness_level = map_range_fromxy_toxy(
            (time_tick.hours * 60 + time_tick.minutes) as f32,
            13.0 * 60.0,
            16.0 * 60.0,
            0.0,
            0.7,
        );
    } else if (hour >= 16 && hour < 20) {
        darkness_level = map_range_fromxy_toxy(
            (time_tick.hours * 60 + time_tick.minutes) as f32,
            16.0 * 60.0,
            20.0 * 60.0,
            0.7,
            0.95,
        );
    } else if (hour >= 20 && hour < 24) {
        darkness_level = map_range_fromxy_toxy(
            (time_tick.hours * 60 + time_tick.minutes) as f32,
            20.0 * 60.0,
            24.0 * 60.0,
            0.95,
            1.0,
        );
    } else if (hour >= 0 && hour < 2) {
        darkness_level = map_range_fromxy_toxy(
            (time_tick.hours * 60 + time_tick.minutes) as f32,
            0.0,
            2.0 * 60.0,
            1.0,
            0.95,
        );
    } else if (hour >= 2 && hour < 4) {
        darkness_level = map_range_fromxy_toxy(
            (time_tick.hours * 60 + time_tick.minutes) as f32,
            2.0 * 60.0,
            4.0 * 60.0,
            0.95,
            0.9,
        );
    } else if (hour >= 4 && hour < 8) {
        darkness_level = map_range_fromxy_toxy(
            (time_tick.hours * 60 + time_tick.minutes) as f32,
            4.0 * 60.0,
            8.0 * 60.0,
            0.9,
            0.0,
        );
    }

    // let darkness_level = map_range_fromxy_toxy(
    //     (time_tick.hours * 60 + time_tick.minutes) as f32,
    //     0.0,
    //     MAX_ABSOLUTE_SECONDS,
    //     0.7,
    //     1.0,
    // );

    if let (Ok((mut cd_colordata, cd_transform))) = colordata_query.get_single_mut() {
        cd_colordata.color.set_a(darkness_level);
    }
}

fn adjust_colordata_via_kb(
    keyboard_input: Res<Input<KeyCode>>,
    mut colordata_query: Query<(&mut CoolMaterialUniformInput, &mut Transform)>,
) {
    for (mut colordata, mut transform) in colordata_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            colordata.position[0].x -= 1. * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::D) {
            colordata.position[0].x += 1. * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::S) {
            colordata.position[0].y -= 1. * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::W) {
            colordata.position[0].y += 1. * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Q) {
            colordata.position[0].z -= 1. * TIME_SKIP * SPEED;
            colordata.position[0].z = colordata.position[0].z.max(0.0);
        } else if keyboard_input.pressed(KeyCode::E) {
            colordata.position[0].z += 1. * TIME_SKIP * SPEED;
        }

        if keyboard_input.pressed(KeyCode::Numpad4) {
            colordata.position[1].x -= 1. * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Numpad6) {
            colordata.position[1].x += 1. * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Numpad2) {
            colordata.position[1].y -= 1. * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Numpad8) {
            colordata.position[1].y += 1. * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Numpad7) {
            colordata.position[1].z -= 10. * TIME_SKIP * SPEED;
            colordata.position[1].z = colordata.position[1].z.max(0.0);
        } else if keyboard_input.pressed(KeyCode::Numpad9) {
            colordata.position[1].z += 10. * TIME_SKIP * SPEED;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 1. * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 1. * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 1. * TIME_SKIP * SPEED;
        } else if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 1. * TIME_SKIP * SPEED;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            let cur_color_a = colordata.color.a();
            colordata.color.set_a(cur_color_a - 0.005);
        } else if keyboard_input.pressed(KeyCode::C) {
            // println!("colordata: {:?}", colordata.color);
            let cur_color_a = colordata.color.a();
            colordata.color.set_a(cur_color_a + 0.005);
        }
        // println!("colordata: {}", colordata.value)
    }
    // println!("");
}
