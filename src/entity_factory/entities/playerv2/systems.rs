// To describe how the Playerv1 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::{Collider, CollidingEntities, RigidBody, Velocity};

use crate::{
    entity_factory::{
        entities::{
            global::{
                ai::{
                    components::{AIDetectionData, AIIdentifier, AIStatus, AITeam},
                    entities::AIEntity,
                },
                collidable::components::{CollidableBody, CollissionData},
                health::components::HealthComponent,
                physics_movable::{
                    components::PXMovableComponent,
                    systems::{insert_physics_components, PhysicsFeature},
                },
                proximity::components::ProximityDataComponent,
                static_movable::components::MovableComponent,
            },
            projectiles::components::{ProjectileEffect, ProjectileEntity, SpawnProjectileEvent},
        },
        factory::data::{GameEntity, SpawnEntityEvent},
    },
    game_modules::{
        camera::components::CameraFollowable, controllable::components::ControllableResource,
        face_axis::components::FaceAxisResource,
    },
};

use super::{components::InputBind, Playerv2Entity};

pub struct Playerv2Plugin;

impl Plugin for Playerv2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(playerv2_move_system)
            .add_system(playerv2_shoot_system);
    }
}

const PLAYER_SIZE: f32 = 10.0;

pub fn plaverv2_spawn(mut commands: &mut Commands, spawn_entity_event: &SpawnEntityEvent) {
    let mut body = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
            ..Default::default()
        },
        transform: Transform {
            translation: spawn_entity_event.position.unwrap_or_default(),
            rotation: spawn_entity_event.rotation.unwrap_or_default(),
            ..Default::default()
        },
        ..Default::default()
    });

    // Base entity
    body.insert(Playerv2Entity)
        .insert(CameraFollowable::default())
        .insert(InputBind {
            active: true,
            mouse_active: true,
        })
        .insert(HealthComponent::default());

    // AI
    body.insert(AIEntity)
        .insert(AIStatus::default())
        .insert(AIIdentifier {
            team: AITeam::Player,
            ..Default::default()
        })
        .insert(AIDetectionData::default());

    // Proximity
    body.insert(ProximityDataComponent {
        triggerer_type: AITeam::Player,

        ..Default::default()
    });

    // Physics
    insert_physics_components(
        &mut body,
        PhysicsFeature {
            size: Some(Vec2::new(PLAYER_SIZE / 2., PLAYER_SIZE / 2.)),
            body_type: Some(CollidableBody::Player),
            ..Default::default()
        },
    );
}

fn playerv2_move_system(
    controller: Res<ControllableResource>,
    faceaxis: Res<FaceAxisResource>,
    mut query: Query<(&InputBind, &mut PXMovableComponent, &Transform), With<Playerv2Entity>>,
) {
    for (input_bind, mut movable, &transform) in query.iter_mut() {
        if (!input_bind.active) {
            return;
        }
        movable.vec_x = controller.joy_x;
        movable.vec_y = controller.joy_y;
        movable.angle = faceaxis.angle;
    }
}

fn playerv2_shoot_system(
    controller: Res<ControllableResource>,
    mut query: Query<(&InputBind, &mut PXMovableComponent, &Transform), With<Playerv2Entity>>,
    mut spawn_projectile_events: EventWriter<SpawnProjectileEvent>,
) {
    for (input_bind, mut movable, &transform) in query.iter_mut() {
        if (!input_bind.active) {
            return;
        }
        if (controller.btn_a.pressed) {
            spawn_projectile_events.send(SpawnProjectileEvent {
                source: Some(transform.translation),
                distance: Some(10.0),
                rotation: Some(transform.rotation),
                projectile_type: ProjectileEntity::Bulletv1,
                effect: Some(ProjectileEffect::Damage { amount: 10.0 }),
                ..Default::default()
            });
        }
    }
}
