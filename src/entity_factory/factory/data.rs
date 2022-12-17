use bevy::{
    ecs::storage::Table,
    prelude::{Component, Quat, Vec3, World},
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
pub enum GameEntity {
    TestBox,
    PlayerV1,
    PlayerV2,
}
