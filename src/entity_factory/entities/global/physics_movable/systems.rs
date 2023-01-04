// To describe how the PhysicsMovable component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::{
    prelude::{
        ActiveEvents, Collider, CollidingEntities, NoUserData, RapierConfiguration,
        RapierPhysicsPlugin, RigidBody, Velocity,
    },
    render::RapierDebugRenderPlugin,
};

use crate::entity_factory::entities::{
    global::collidable::components::{CollidableBody, CollissionData},
    projectiles::components::ProjectileEffect,
};

use super::{
    components::{PXConstantMovement, PXSize},
    PXMovableComponent,
};

pub struct PhysicsMovablePlugin;

impl Plugin for PhysicsMovablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.0))
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(physics_init_system)
            .add_system(physics_movement_system);
    }
}

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 50.;

pub struct PhysicsFeature {
    pub size: Option<Vec2>,
    pub paths_data: Option<(Vec<Vec2>, Vec<[u32; 2]>)>,
    pub body_type: Option<CollidableBody>,
    pub effect: Option<ProjectileEffect>,
    pub rigidbody_type: Option<RigidBody>,
    pub record_collidability: bool,
}

impl Default for PhysicsFeature {
    fn default() -> Self {
        Self {
            size: None,
            paths_data: None,
            body_type: None,
            effect: None,
            rigidbody_type: Some(RigidBody::Dynamic),
            record_collidability: false,
        }
    }
}

pub fn insert_physics_components(ent_com: &mut EntityCommands, features: PhysicsFeature) {
    let size = features.size.unwrap_or(Vec2::ZERO);

    ent_com
        .insert(CollidingEntities::default())
        .insert(CollissionData {
            body: features.body_type.unwrap_or(CollidableBody::Base),
            effect: features.effect,
            ..Default::default()
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(features.rigidbody_type.unwrap_or_default())
        .insert(Velocity::zero())
        .insert(PXMovableComponent::default());
    if features.record_collidability {
        ent_com.insert(PXSize {
            width: size.x * 2.,
            height: size.y * 2.,
        });
    }

    if (features.paths_data.is_some()) {
        let (paths, indices) = features.paths_data.unwrap();
        ent_com.insert(Collider::polyline(paths, Some(indices)));
    } else {
        ent_com.insert(Collider::cuboid(size.x, size.y));
    }
}

fn physics_init_system(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

fn physics_movement_system(mut query: Query<(&PXMovableComponent, &mut Velocity, &mut Transform)>) {
    for (input_movable, mut rb_vels, mut tf) in query.iter_mut() {
        let mut move_delta = Vec2::new(input_movable.vec_x, input_movable.vec_y);
        // if move_delta != Vec2::ZERO {
        // move_delta /= move_delta.length();
        // }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta * BASE_SPEED;
        // tf.rotate(Quat::from_rotation_z(1.0 * TIME_STEP));
        tf.rotation = Quat::from_rotation_z(input_movable.angle);

        // get current angle of the rigid body
        // let angle = tf.rotation.to_axis_angle().1;
        // get the angle between the current angle and the target angle
        // let angle_diff = input_movable.angle - angle;

        // if the angle is greater than 0.1, rotate the rigid body
        // if angle_diff.abs() > 0.1 {
        // tf.rotate(Quat::from_rotation_z(angle_diff * TIME_STEP));
        // }
    }
}
