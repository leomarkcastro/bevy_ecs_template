// To describe how the Polygonv2 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::prelude::{DrawMode, FillMode, GeometryBuilder, PathBuilder, StrokeMode};
use bevy_rapier2d::prelude::RigidBody;

use crate::entity_factory::{
    entities::global::{
        collidable::components::CollidableBody,
        despawn::components::DespawnComponent,
        physics_movable::systems::{insert_physics_components, PhysicsFeature},
    },
    factory::data::{GameEntityData, SpawnEntityEvent},
};

use super::Polygonv2Entity;

pub struct Polygonv2Plugin;

impl Plugin for Polygonv2Plugin {
    fn build(&self, app: &mut App) {}
}

pub fn polygonv2_spawn(mut body: &mut EntityCommands, spawn_entity_event: &SpawnEntityEvent) {
    let data = spawn_entity_event.entity_data.as_ref();

    match data {
        Some(GameEntityData::Polygonv2 {
            path,
            despawn,
            style,
            is_collidable,
        }) => {
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
                style.to_owned(),
                Transform {
                    translation: spawn_entity_event.position.unwrap_or_default(),
                    rotation: spawn_entity_event.rotation.unwrap_or_default(),
                    ..Default::default()
                },
            ));

            // Base entity
            body.insert(Polygonv2Entity).insert(DespawnComponent {
                bldg_circle: despawn.bldg_circle,
                camera_circle: despawn.camera_circle,
                id: despawn.id.clone(),
            });

            if (is_collidable == &true) {
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
        }
        _ => {}
    }
}
