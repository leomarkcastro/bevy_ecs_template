// To describe how the Polygonv1 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::prelude::{DrawMode, FillMode, GeometryBuilder, PathBuilder, StrokeMode};
use bevy_rapier2d::prelude::RigidBody;

use crate::entity_factory::{
    entities::global::{
        collidable::components::CollidableBody,
        physics_movable::systems::{insert_physics_components, PhysicsFeature},
    },
    factory::data::{GameEntityData, SpawnEntityEvent},
};

use super::Polygonv1Entity;

pub struct Polygonv1Plugin;

impl Plugin for Polygonv1Plugin {
    fn build(&self, app: &mut App) {}
}

pub fn polygonv1_spawn(mut body: &mut EntityCommands, spawn_entity_event: &SpawnEntityEvent) {
    // let box_size = spawn_entity_event.size.unwrap_or_default();
    // let mut body = commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(0.0, 1.0, 0.0),
    //         custom_size: Some(box_size),
    //         ..Default::default()
    //     },
    //     transform: Transform {
    //         translation: spawn_entity_event.position.unwrap_or_default(),
    //         rotation: spawn_entity_event.rotation.unwrap_or_default(),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });

    let data = spawn_entity_event.entity_data.as_ref();

    match data {
        Some(GameEntityData::Polygonv1 { path, border_size }) => {
            let mut path_builder = PathBuilder::new();
            path_builder.move_to(path[0]);
            for pts in path.iter().skip(1) {
                path_builder.line_to(Vec2::new(pts.x, pts.y));
            }
            let line = path_builder.build();

            let mut index = 0;
            let mut index_list = Vec::new();
            for _ in path.iter().skip(1) {
                index_list.push([index, index + 1]);
                index += 1;
            }

            body.insert(GeometryBuilder::build_as(
                &line,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GREEN),
                    outline_mode: StrokeMode::new(Color::DARK_GREEN, border_size.to_owned()),
                },
                Transform {
                    translation: spawn_entity_event.position.unwrap_or_default(),
                    rotation: spawn_entity_event.rotation.unwrap_or_default(),
                    ..Default::default()
                },
            ));
            // .insert(Collider::convex_decomposition(
            //     mountain_1_points.as_slice(),
            //     index_list.as_slice(),
            // ))
            // .insert(Collider::polyline(mountain_1_points, Some(index_list)));

            // Base entity
            body.insert(Polygonv1Entity);

            // Physics
            insert_physics_components(
                &mut body,
                PhysicsFeature {
                    paths_data: Some((path.clone(), index_list)),
                    body_type: Some(CollidableBody::Block),
                    rigidbody_type: Some(RigidBody::Fixed),
                    ..Default::default()
                },
            );
        }
        _ => {}
    }
}
