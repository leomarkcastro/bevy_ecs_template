// To describe how the Zombiesv2 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Anchor};

use crate::{
    entity_factory::{
        entities::global::{
            ai::{
                components::{AIIdentifier, AIMode, AIStatus, AITeam},
                entities::AIEntity,
            },
            animated::components::AnimatedComponent,
            collidable::components::CollidableBody,
            despawn::components::DespawnComponent,
            health::components::HealthComponent,
            physics_movable::{
                components::PXMovableComponent,
                systems::{insert_physics_components, PhysicsFeature},
            },
        },
        factory::data::{GameEntityData, SpawnEntityEvent},
    },
    game_modules::{
        dynamic_data::animation_definition::AnimationDefinition,
        load_assets::{
            components::{insert_animation_components, AnimationSettings},
            systems::GameTextures,
        },
    },
};

use super::{Zombiesv2Component, Zombiesv2Entity};

pub struct Zombiesv2Plugin;

impl Plugin for Zombiesv2Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_zombie_animations)
            .add_system(zombiev2_anim_state_system);
    }
}

#[derive(Resource)]
pub struct ZombieAnimDefinitionHandle(pub Handle<AnimationDefinition>);

fn load_zombie_animations(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game_textures: ResMut<GameTextures>,
) {
    let def_data: Handle<AnimationDefinition> =
        assets_server.load("image_sprite_zombie/data.animation.json");
    commands.insert_resource(ZombieAnimDefinitionHandle(def_data));

    let texture_handle = assets_server.load("image_sprite_zombie/atlas.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(512., 512.), 9, 6, None, None);
    let zombie_atlas = texture_atlases.add(texture_atlas);

    // game_textures.player =
    //     Some(assets_server.load("image_sprite_humans/idle_gun/Idle_gun_000.png"));
    game_textures.zombie_atlas = Some(zombie_atlas);
}

fn zombiev2_anim_state_system(
    mut query: Query<(&mut PXMovableComponent, &mut AnimatedComponent), With<Zombiesv2Entity>>,
    anim_def_handle: Res<ZombieAnimDefinitionHandle>,
    mut animation_definitions: ResMut<Assets<AnimationDefinition>>,
) {
    for (mut movable, mut anim_comp) in query.iter_mut() {
        // get the contents of the handle
        if let Some(anim_def) = animation_definitions.get(&anim_def_handle.0) {
            // Female Zombie = 0
            // Male Zombie   = 27
            let base = 0;

            // + 0 attack
            // + 9 dead
            // + 18 walk
            let modifier = 18;

            let is_moving = if (movable.vec_x == 0.0 && movable.vec_y == 0.0) {
                0
            } else {
                1
            };

            anim_comp.index_start = base + modifier;

            let current_index = (anim_comp.index_start) / 9;
            // current_index
            let current_animation = anim_def.animation_keys.get(current_index as usize).unwrap();
            let anim_data = anim_def.metadata.get(current_animation).unwrap();

            let index_end = anim_comp.index_start + anim_data.total_sprites * is_moving;

            anim_comp.index_end = index_end;
            // println!(
            //     "{} -[{}]-> {}",
            //     anim_comp.index_start,
            //     anim_data.total_sprites * is_moving,
            //     anim_comp.index_end
            // );
        }
    }
}

const SPRITE_SIZE_X: f32 = 40.0;
const SPRITE_SIZE_Y: f32 = 40.0;
const COLLISSION_SIZE_X: f32 = 5.0;
const COLLISSION_SIZE_Y: f32 = 6.0;

pub fn zombiesv2_spawn(
    mut body: &mut EntityCommands,
    spawn_entity_event: &SpawnEntityEvent,
    game_textures: &Res<GameTextures>,
) {
    // return; // TODO: remove this
    let data = spawn_entity_event.entity_data.as_ref();
    let mut _despawn_data = None;
    if let Some(GameEntityData::Zombiesv2 { despawn_data }) = data {
        _despawn_data = Some(despawn_data.clone());
    }

    body.insert(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            custom_size: Some(Vec2::new(SPRITE_SIZE_X, SPRITE_SIZE_Y)),
            index: 0,
            anchor: Anchor::Custom(Vec2 { x: 0.18, y: 0.0 }),
            ..Default::default()
        },
        texture_atlas: game_textures.zombie_atlas.as_ref().unwrap().clone(),
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
            index_start: 18,
            index_end: 18 + 1,
            ..Default::default()
        }),
    );

    // Base entity
    body.insert(Zombiesv2Entity).insert(HealthComponent {
        health: 50.0,
        ..Default::default()
    });

    // Physics
    insert_physics_components(
        &mut body,
        PhysicsFeature {
            size: Some(Vec2::new(COLLISSION_SIZE_X, COLLISSION_SIZE_Y)),
            body_type: Some(CollidableBody::Enemy),
            ..Default::default()
        },
    );

    // Zombiesv2
    // AI
    body.insert(AIEntity)
        .insert(AIStatus {
            active: true,
            ..Default::default()
        })
        .insert(AIIdentifier {
            team: AITeam::Zombies,
            ..Default::default()
        });

    if _despawn_data.is_some() {
        let dd = _despawn_data.unwrap();
        // Dissapear after sitance
        body.insert(DespawnComponent {
            bldg_circle: dd.bldg_circle,
            camera_circle: dd.camera_circle,
            id: dd.id.clone(),
        });
    }
}
