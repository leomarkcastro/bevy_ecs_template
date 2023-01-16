// To describe how the Bulletv2 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::{CollidingEntities, Sensor, Velocity};

use crate::entity_factory::{
    entities::{
        global::{
            collidable::components::CollidableBody,
            physics_movable::{
                components::{PXConstantMovement, PXMovableComponent},
                systems::{insert_physics_components, PhysicsFeature},
            },
        },
        projectiles::{
            components::{ProjectileEffect, SpawnProjectileEvent},
            projectile_entities::bulletv1::components::DissapearAtDistance,
        },
    },
    factory::data::SpawnEntityEvent,
};

use super::Bulletv2Entity;

pub struct Bulletv2Plugin;

impl Plugin for Bulletv2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(bulletv2_system);
    }
}

pub fn bulletv2_spawn(
    mut body: &mut EntityCommands,
    spawn_entity_event: &SpawnProjectileEvent,
    asset_server: &Res<AssetServer>,
) {
    // move at asset from current position
    let transform_xy = spawn_entity_event.source.unwrap_or_default();

    let distance = spawn_entity_event.distance.unwrap_or_default();

    let rotation = spawn_entity_event.rotation.unwrap_or_default();

    // quat to vec3
    let transform_angle = rotation.to_axis_angle().1 * rotation.to_axis_angle().0.z;

    // get new position with x distrance from target asset and angle
    let transform_xy_with_angle = transform_xy.xy()
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
            custom_size: Some(Vec2::new(10.0, 5.0)),
            ..Default::default()
        },
        texture: asset_server.load("image_effects/shot.png"),
        transform: Transform {
            translation: transform_xy_with_angle.extend(transform_xy.z),
            rotation: rotation,
            ..Default::default()
        },
        ..Default::default()
    });

    body.insert(Bulletv2Entity).insert(DissapearAtDistance {
        max_distance: 100.0,
        current_distance: 0.0,
    });

    insert_physics_components(
        &mut body,
        PhysicsFeature {
            size: Some(Vec2::new(0.5, 0.5)),
            body_type: Some(CollidableBody::Bullet),
            effect: spawn_entity_event.effect,
            modify_angle: false,
            ..Default::default()
        },
    );

    body.insert(PXConstantMovement {
        speed: 10.0,
        angle: transform_angle,
    });

    // Sensor only
    body.insert(Sensor);
}

const BULLET_SPEED: f32 = 10.;
const MULTIPLIER: f32 = 1.;

fn bulletv2_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut PXMovableComponent,
            &PXConstantMovement,
            &mut DissapearAtDistance,
            &Transform,
        ),
        With<Bulletv2Entity>,
    >,
) {
    // // Single Query
    // if let Ok(mut bulletv2_component) = query.get_single_mut() {
    //     bulletv2_component.data = "Hello, World!".to_string();
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
