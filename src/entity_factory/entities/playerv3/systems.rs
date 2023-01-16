// To describe how the Playerv1 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, math::Vec3Swizzles, prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::{Collider, CollidingEntities, RigidBody, Velocity};

use crate::{
    entity_factory::{
        entities::{
            global::{
                ai::{
                    components::{AIDetectionData, AIIdentifier, AIStatus, AITeam},
                    entities::AIEntity,
                },
                animated::components::AnimatedComponent,
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
            zombiesv2::systems::ZombieAnimDefinitionHandle,
        },
        factory::data::{GameEntity, SpawnEntityEvent},
    },
    game_modules::{
        camera::components::CameraFollowable,
        controllable::components::ControllableResource,
        dynamic_data::animation_definition::AnimationDefinition,
        face_axis::components::FaceAxisResource,
        load_assets::{
            components::{insert_animation_components, Animated, AnimationSettings},
            systems::GameTextures,
        },
        timers::components::{HalfMilliSecondTimer, MillisencondTimer, QuarterSencondTimer},
    },
};

use super::{components::InputBind, Playerv3Entity};

#[derive(Resource)]
pub struct PlayerAnimDefinitionHandle(pub Handle<AnimationDefinition>);

pub struct Playerv3Plugin;

impl Plugin for Playerv3Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_player_animations)
            .add_system(playerv3_move_system)
            .add_system(playerv3_shoot_system)
            .add_system(playerv3_anim_state_system);
    }
}

const SPRITE_SIZE_X: f32 = 40.0;
const SPRITE_SIZE_Y: f32 = 40.0;
const COLLISSION_SIZE_X: f32 = 5.0;
const COLLISSION_SIZE_Y: f32 = 6.0;

fn load_player_animations(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game_textures: ResMut<GameTextures>,
) {
    let def_data: Handle<AnimationDefinition> =
        assets_server.load("image_sprite_humans/data.animation.json");
    commands.insert_resource(PlayerAnimDefinitionHandle(def_data));

    let texture_handle = assets_server.load("image_sprite_humans/atlas.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(512., 512.), 12, 16, None, None);
    let player_atlas = texture_atlases.add(texture_atlas);

    // game_textures.player =
    //     Some(assets_server.load("image_sprite_humans/idle_gun/Idle_gun_000.png"));
    game_textures.player_atlas = Some(player_atlas);
}

pub fn plaverv3_spawn(
    mut body: &mut EntityCommands,
    spawn_entity_event: &SpawnEntityEvent,
    game_textures: &Res<GameTextures>,
) {
    body.insert(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            // color: Color::rgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(SPRITE_SIZE_X, SPRITE_SIZE_Y)),
            index: 96,
            anchor: Anchor::Custom(Vec2 { x: -0.25, y: 0.05 }),
            ..Default::default()
        },
        texture_atlas: game_textures.player_atlas.as_ref().unwrap().clone(),
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
            frame_duration: 0.12,
            index_start: 96,
            index_end: 96 + 8,
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
        .insert(HealthComponent {
            health: 1000000.0, // TODO: Change to 100
            ..Default::default()
        });

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
            size: Some(Vec2::new(COLLISSION_SIZE_X, COLLISSION_SIZE_Y)),
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
    time: Res<Time>,
    mut interval_timer: ResMut<MillisencondTimer>,
) {
    for (input_bind, mut movable, &transform) in query.iter_mut() {
        if (!input_bind.active) {
            return;
        }
        if (controller.btn_a.hold) {
            // move source to the front of the player
            if interval_timer.event_timer.tick(time.delta()).finished() {
                let source =
                    transform.translation + transform.rotation * Vec3::new(1.0, -2.0, 250.0);
                spawn_projectile_events.send(SpawnProjectileEvent {
                    source: Some(source),
                    distance: Some(10.0),
                    rotation: Some(transform.rotation),
                    projectile_type: ProjectileEntity::Bulletv2,
                    effect: Some(ProjectileEffect::Damage { amount: 10.0 }),

                    ..Default::default()
                });
            }
        }
    }
}

fn playerv3_anim_state_system(
    mut query: Query<(&mut PXMovableComponent, &mut AnimatedComponent), With<Playerv3Entity>>,
    anim_def_handle: Res<PlayerAnimDefinitionHandle>,
    mut animation_definitions: ResMut<Assets<AnimationDefinition>>,
) {
    for (mut movable, mut anim_comp) in query.iter_mut() {
        // get the contents of the handle
        if let Some(anim_def) = animation_definitions.get(&anim_def_handle.0) {
            // shooting = 12
            // idle     = 72
            // moving   =
            let base = if (movable.vec_x == 0.0 && movable.vec_y == 0.0) {
                72
            } else {
                132
            };

            // + 0 bat
            // + 12 flame
            // + 24 gun
            // + 36 knife
            // + 48 rifle
            let weapons = 24;

            if anim_comp.index_start == base + weapons {
                continue;
            }

            anim_comp.index_start = base + weapons;

            let current_index = (anim_comp.index_start) / 12;
            // current_index
            let current_animation = anim_def.animation_keys.get(current_index as usize).unwrap();
            let anim_data = anim_def.metadata.get(current_animation).unwrap();

            anim_comp.index_end = anim_comp.index_start + anim_data.total_sprites;
        }
    }
}

fn test_playerv3_changeanim_system(
    controller: Res<ControllableResource>,
    mut query: Query<
        (
            &InputBind,
            &mut PXMovableComponent,
            &Transform,
            &mut AnimatedComponent,
        ),
        With<Playerv3Entity>,
    >,
    anim_def_handle: Res<PlayerAnimDefinitionHandle>,
    mut animation_definitions: ResMut<Assets<AnimationDefinition>>,
) {
    for (input_bind, mut movable, &transform, mut anim_comp) in query.iter_mut() {
        if (!input_bind.active) {
            return;
        }
        if (controller.btn_a.pressed) {
            // get the contents of the handle
            if let Some(anim_def) = animation_definitions.get(&anim_def_handle.0) {
                anim_comp.index_start += 12;
                if (anim_comp.index_start >= 12 * 16) {
                    anim_comp.index_start = 0;
                }

                let current_index = (anim_comp.index_start) / 12;
                // current_index
                let current_animation =
                    anim_def.animation_keys.get(current_index as usize).unwrap();
                let anim_data = anim_def.metadata.get(current_animation).unwrap();

                anim_comp.index_end = anim_comp.index_start + anim_data.total_sprites;
            }
        }
    }
}
