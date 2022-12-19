// To describe how the SimplePointLight component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{
    prelude::{shape::Quad, *},
    render::{
        render_resource::{encase, OwnedBindingResource},
        renderer::RenderQueue,
        Extract, RenderApp, RenderStage,
    },
    sprite::{Material2dPlugin, MaterialMesh2dBundle, RenderMaterials2d},
};
use bevy_inspector_egui::RegisterInspectable;

use crate::game_modules::camera::systems::{PROJECTION_SIZE, RESOLUTION};

use super::components::{CoolMaterial, CoolMaterialUniformBuffer, CoolMaterialUniformInput};

pub struct SimplePointLightPlugin;

impl Plugin for SimplePointLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(simple_point_light_init_system)
            .add_plugin(Material2dPlugin::<CoolMaterial>::default())
            .register_inspectable::<CoolMaterialUniformInput>();
        // .add_system(adjust_colordata_via_kb);

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
                    size: Vec2::new(OVERLAY_SIZE[0], OVERLAY_SIZE[1]),
                    ..Default::default()
                })))
                .into(),
            material: my_material_assets.add(CoolMaterial {
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
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
            colordata.color.set_a(cur_color_a - 0.0005);
        } else if keyboard_input.pressed(KeyCode::X) {
            let cur_color_a = colordata.color.a();
            colordata.color.set_a(cur_color_a + 0.0005);
        }
        // println!("colordata: {}", colordata.value)
    }
    // println!("");
}
