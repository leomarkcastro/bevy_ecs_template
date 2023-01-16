use bevy::prelude::{Component, Quat, Vec3, World};

// To be used as data for the projectiles entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Debug)]
pub struct SpawnProjectileEvent {
    pub projectile_type: ProjectileEntity,
    pub source: Option<Vec3>,
    pub distance: Option<f32>,
    pub rotation: Option<Quat>,
    pub spawn_at: World,
    pub effect: Option<ProjectileEffect>,
}

impl Default for SpawnProjectileEvent {
    fn default() -> Self {
        Self {
            projectile_type: ProjectileEntity::Bulletv1,
            source: None,
            distance: None,
            rotation: None,
            spawn_at: World::default(),
            effect: None,
        }
    }
}

#[derive(Debug)]
pub enum ProjectileEntity {
    Bulletv1,
    Bulletv2,
}

#[derive(Debug, Copy, Clone)]
pub enum ProjectileEffect {
    Damage { amount: f32 },
    Heal { amount: f32 },
    None,
}
