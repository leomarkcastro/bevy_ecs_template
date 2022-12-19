use bevy::{
    prelude::{Color, Component, Vec4},
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::Material2d,
};
use bevy_inspector_egui::Inspectable;

// To be used as data for the simple_point_light entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

// create n amount of vec4s using macro
macro_rules! vec4s {
    ($n:expr) => {
        [Vec4::default(); $n]
    };
}

const MAX_LIGHTS: usize = 64;

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CoolMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub position: [Vec4; MAX_LIGHTS],
}

impl Default for CoolMaterial {
    fn default() -> Self {
        Self {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            position: vec4s!(MAX_LIGHTS),
        }
    }
}

impl Material2d for CoolMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/simple_point_light.wgsl".into()
    }
}

#[derive(Clone, ShaderType)]
pub struct CoolMaterialUniformBuffer {
    pub color: Color,
    pub position: [Vec4; MAX_LIGHTS],
}

#[derive(Component, Clone, Copy, Inspectable)]
pub struct CoolMaterialUniformInput {
    pub color: Color,
    pub position: [Vec4; MAX_LIGHTS],
}

impl Default for CoolMaterialUniformInput {
    fn default() -> Self {
        Self {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            position: vec4s!(MAX_LIGHTS),
        }
    }
}
