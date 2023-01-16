// To describe how the Projectiles component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::{
    components::{ProjectileEntity, SpawnProjectileEvent},
    projectile_entities::{
        bulletv1::systems::{bulletv1_spawn, Bulletv1Plugin},
        bulletv2::systems::{bulletv2_spawn, Bulletv2Plugin},
    },
};

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnProjectileEvent>()
            .add_plugin(Bulletv1Plugin)
            .add_plugin(Bulletv2Plugin)
            .add_system(projectiles_system);
    }
}

fn projectiles_system(
    mut commands: Commands,
    mut spawn_entity_events: EventReader<SpawnProjectileEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in spawn_entity_events.iter() {
        let mut basebody = commands.spawn_empty();
        match event.projectile_type {
            ProjectileEntity::Bulletv1 => bulletv1_spawn(&mut basebody, event),
            ProjectileEntity::Bulletv2 => bulletv2_spawn(&mut basebody, event, &asset_server),
            _ => {}
        }
    }
}
