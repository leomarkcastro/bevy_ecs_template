// To describe how the Playerv1 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, math::Vec3Swizzles, prelude::*};
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
        camera::components::CameraFollowable,
        controllable::components::ControllableResource,
        face_axis::components::FaceAxisResource,
        load_assets::{
            components::{insert_animation_components, Animated, AnimationSettings},
            systems::GameTextures,
        },
    },
};

use super::{components::InputBind, Playerv3Entity};

pub struct Playerv3Plugin;

impl Plugin for Playerv3Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_player_animations)
            .add_system(playerv3_move_system)
            .add_system(playerv3_shoot_system);
    }
}

const PLAYER_SIZE_X: f32 = 13.0;
const PLAYER_SIZE_Y: f32 = 10.0;

fn load_player_animations(
    assets_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = assets_server.load("image_sprite_humans/idle_gun.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(423., 349.), 8, 1, None, None);
    let explosion = texture_atlases.add(texture_atlas);
}

pub fn plaverv3_spawn(
    mut body: &mut EntityCommands,
    spawn_entity_event: &SpawnEntityEvent,
    game_textures: &Res<GameTextures>,
) {
    body.insert(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            // color: Color::rgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(PLAYER_SIZE_X, PLAYER_SIZE_Y)),
            ..Default::default()
        },
        texture_atlas: game_textures.idle_gun.clone(),
        transform: Transform {
            translation: spawn_entity_event.position.unwrap_or_default(),
            rotation: spawn_entity_event.rotation.unwrap_or_default(),
            ..Default::default()
        },
        ..Default::default()
    });

    insert_animation_components(
        &mut body,
        Some(AnimationSettings {
            frame_duration: 0.2,
            ..Default::default()
        }),
    );

    // Base entity
    body.insert(Playerv3Entity)
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
            size: Some(Vec2::new(PLAYER_SIZE_X / 2., PLAYER_SIZE_Y / 2.)),
            body_type: Some(CollidableBody::Player),
            ..Default::default()
        },
    );
}

fn playerv3_move_system(
    controller: Res<ControllableResource>,
    faceaxis: Res<FaceAxisResource>,
    mut query: Query<(&InputBind, &mut PXMovableComponent, &Transform), With<Playerv3Entity>>,
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

fn playerv3_shoot_system(
    controller: Res<ControllableResource>,
    mut query: Query<(&InputBind, &mut PXMovableComponent, &Transform), With<Playerv3Entity>>,
    mut spawn_projectile_events: EventWriter<SpawnProjectileEvent>,
) {
    for (input_bind, mut movable, &transform) in query.iter_mut() {
        if (!input_bind.active) {
            return;
        }
        if (controller.btn_a.pressed) {
            // move source to the front of the player
            let source = transform.translation + transform.rotation * Vec3::new(1.0, -2.0, 0.0);
            spawn_projectile_events.send(SpawnProjectileEvent {
                source: Some(source),
                distance: Some(10.0),
                rotation: Some(transform.rotation),
                projectile_type: ProjectileEntity::Bulletv1,
                effect: Some(ProjectileEffect::Damage { amount: 10.0 }),

                ..Default::default()
            });
        }
    }
}
