use bevy::{prelude::Component, utils::Uuid};

use crate::entity_factory::entities::projectiles::components::ProjectileEffect;

// To be used as data for the collidable entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Debug)]
pub enum CollidableBody {
    Bullet,
    Player,
    Enemy,
    Block,
    Artifacts,
    Base,
}

#[derive(Debug, Component)]
pub struct CollissionData {
    pub body: CollidableBody,
    pub uuid: Uuid,
    pub effect: Option<ProjectileEffect>,
}

impl Default for CollissionData {
    fn default() -> Self {
        Self {
            body: CollidableBody::Base,
            uuid: Uuid::new_v4(),
            effect: None,
        }
    }
}
