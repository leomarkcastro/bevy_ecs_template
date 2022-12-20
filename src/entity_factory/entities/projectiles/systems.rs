// To describe how the Projectiles component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::{
    components::{ProjectileEntity, SpawnProjectileEvent},
    projectile_entities::bulletv1::systems::{bulletv1_spawn, Bulletv1Plugin},
};

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnProjectileEvent>()
            .add_plugin(Bulletv1Plugin)
            .add_system(projectiles_system);
    }
}

fn projectiles_system(
    mut commands: Commands,
    mut spawn_entity_events: EventReader<SpawnProjectileEvent>,
) {
    for event in spawn_entity_events.iter() {
        match event.projectile_type {
            ProjectileEntity::Bulletv1 => bulletv1_spawn(&mut commands, event),
            _ => {}
        }
    }
}
