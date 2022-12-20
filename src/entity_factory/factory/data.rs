use bevy::{
    ecs::storage::Table,
    prelude::{Color, Component, Quat, Vec3, World},
    text::TextAlignment,
    ui::{PositionType, Style, UiRect, Val},
};

use crate::scene_manager::manager::{entities::SpawnAt, scene_list::GameScene};

#[derive(Debug)]
pub struct SpawnEntityEvent {
    pub entity: GameEntity,
    pub position: Option<Vec3>,
    pub rotation: Option<Quat>,
    pub spawn_at: World,
}

impl Default for SpawnEntityEvent {
    fn default() -> Self {
        Self {
            entity: GameEntity::TestBox,
            position: Some(Vec3::new(0.0, 0.0, 0.0)),
            rotation: Some(
                Quat::from_rotation_x(0.0)
                    * Quat::from_rotation_y(0.0)
                    * Quat::from_rotation_z(0.0),
            ),
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
    Bulletv1,
    Zombiesv1,
}

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
