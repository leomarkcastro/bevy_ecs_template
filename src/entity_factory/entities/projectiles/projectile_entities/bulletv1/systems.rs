// To describe how the Bulletv1 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::{CollidingEntities, Velocity};

use crate::entity_factory::{
    entities::{
        global::{
            collidable::components::CollidableBody,
            physics_movable::{
                components::{PXConstantMovement, PXMovableComponent},
                systems::{insert_physics_components, PhysicsFeature},
            },
        },
        projectiles::components::{ProjectileEffect, SpawnProjectileEvent},
    },
    factory::data::SpawnEntityEvent,
};

use super::{components::DissapearAtDistance, Bulletv1Component, Bulletv1Entity};

pub struct Bulletv1Plugin;

impl Plugin for Bulletv1Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(bulletv1_system);
    }
}

pub fn bulletv1_spawn(mut body: &mut EntityCommands, spawn_entity_event: &SpawnProjectileEvent) {
    // move at asset from current position
    let transform_xy = spawn_entity_event.source.unwrap_or_default().xyy().xy();

    let distance = spawn_entity_event.distance.unwrap_or_default();

    let rotation = spawn_entity_event.rotation.unwrap_or_default();

    // quat to vec3
    let transform_angle = rotation.to_axis_angle().1 * rotation.to_axis_angle().0.z;

    // get new position with x distrance from target asset and angle
    let transform_xy_with_angle = transform_xy
        + Vec2::new(
            distance * transform_angle.cos(),
            distance * transform_angle.sin(),
        );

    // println!(
    //     "spawn {:?}",
    //     spawn_entity_event.rotation.unwrap_or_default()
    // );
    body.insert(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: transform_xy_with_angle.extend(40.0),
            rotation: rotation,
            ..Default::default()
        },
        ..Default::default()
    });

    body.insert(Bulletv1Entity).insert(DissapearAtDistance {
        max_distance: 100.0,
        current_distance: 0.0,
    });

    insert_physics_components(
        &mut body,
        PhysicsFeature {
            size: Some(Vec2::new(0.5, 0.5)),
            body_type: Some(CollidableBody::Bullet),
            effect: spawn_entity_event.effect,
            ..Default::default()
        },
    );

    body.insert(PXConstantMovement {
        speed: 10.0,
        angle: transform_angle,
    });
}

const BULLET_SPEED: f32 = 10.;
const MULTIPLIER: f32 = 1.;

fn bulletv1_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut PXMovableComponent,
            &PXConstantMovement,
            &mut DissapearAtDistance,
            &Transform,
        ),
        With<Bulletv1Entity>,
    >,
) {
    // // Single Query
    // if let Ok(mut bulletv1_component) = query.get_single_mut() {
    //     bulletv1_component.data = "Hello, World!".to_string();
    // }

    // // Multiple Queries
    for (entity, mut velocity, &movement, mut dissapear_at_distance, &transform) in query.iter_mut()
    {
        // get angle
        let angle = movement.angle;

        // get velocity
        velocity.vec_x = angle.cos() * BULLET_SPEED * MULTIPLIER;
        velocity.vec_y = angle.sin() * BULLET_SPEED * MULTIPLIER;

        // println!("velocity: {:?}", velocity);

        let distance_covered =
            (velocity.vec_x * velocity.vec_x + velocity.vec_y * velocity.vec_y).sqrt();

        dissapear_at_distance.current_distance += distance_covered;

        // if distance travelled is greater than max distance, then delete entity
        if dissapear_at_distance.current_distance > dissapear_at_distance.max_distance {
            //     // despawn entity
            commands.entity(entity).despawn();
        }
    }
}
