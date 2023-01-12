use bevy::{
    ecs::storage::Table,
    prelude::{Color, Component, Quat, Vec2, Vec3, World},
    text::TextAlignment,
    ui::{PositionType, Style, UiRect, Val},
    utils::Uuid,
};
use bevy_prototype_lyon::prelude::DrawMode;

use crate::{
    entity_factory::entities::global::{
        despawn::components::DespawnComponent,
        despawn_on_clock::components::DespawnWithTimerComponent,
    },
    game_modules::{
        global_event::systems::GlobalEvent,
        map_loader::data::{RoomData, RoomType},
    },
    scene_manager::manager::{entities::SpawnAt, scene_list::GameScene},
};

#[derive(Debug)]
pub struct SpawnEntityEvent {
    pub id: Uuid,
    pub entity: GameEntity,
    pub entity_data: Option<GameEntityData>,
    pub position: Option<Vec3>,
    pub rotation: Option<Quat>,
    pub size: Option<Vec2>,
    pub spawn_at: World,
}

impl Default for SpawnEntityEvent {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            entity: GameEntity::TestBox,
            entity_data: None,
            position: Some(Vec3::new(0.0, 0.0, 0.0)),
            rotation: Some(
                Quat::from_rotation_x(0.0)
                    * Quat::from_rotation_y(0.0)
                    * Quat::from_rotation_z(0.0),
            ),
            size: Some(Vec2::new(10.0, 10.0)),
            spawn_at: World::default(),
        }
    }
}

#[derive(Debug)]
pub struct SpawnUIEvent {
    pub entity: UIEntity,
    pub entitydata: UIEntityData,
    pub style: Style,
    pub spawn_at: World,
}

impl Default for SpawnUIEvent {
    fn default() -> Self {
        Self {
            entity: UIEntity::SimpleText,
            entitydata: UIEntityData::SimpleText {
                text: "UIText".to_string(),
                font: "fonts/FiraSans-Bold.ttf".to_string(),
                font_size: 50.0,
                alignment: TextAlignment::TOP_CENTER,
                color: Color::BLACK,
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },

            spawn_at: World::default(),
        }
    }
}

#[derive(Debug)]
pub enum GameEntity {
    TestBox,
    PlayerV1,
    PlayerV2,
    PlayerV3,
    Bulletv1,
    Zombiesv1,
    Pickupablev1,
    Blockv1,
    Blockv2,
    Blockv3,
    Polygonv1,
    Polygonv2,
    Roomv1,
    Roofv1,
    Treev1,
}

#[derive(Debug)]
pub enum GameEntityData {
    Pickupablev1 {
        on_pickup: GlobalEvent,
    },
    Blockv3 {
        despawn_data: DespawnComponent,
    },
    Polygonv1 {
        path: Vec<Vec2>,
        border_size: f32,
    },
    Polygonv2 {
        path: Vec<Vec2>,
        despawn: DespawnComponent,
        style: DrawMode,
        is_collidable: bool,
    },
    Roomv1 {
        room_type: RoomType,
        despawn_data: DespawnComponent,
    },
    Treev1 {
        despawn_data: DespawnComponent,
        internal_radius_percentage: f32,
    },
    Block {
        no_physic: bool,
        despawn_timer_data: DespawnWithTimerComponent,
    },
    Zombiesv1 {
        despawn_data: DespawnComponent,
    },
}

// ==================================

#[derive(Debug)]
pub enum UIEntity {
    SimpleText,
}

#[derive(Debug)]
pub enum UIEntityData {
    SimpleText {
        text: String,
        font: String,
        font_size: f32,
        color: Color,
        alignment: TextAlignment,
    },
}
