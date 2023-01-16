// To describe how the LoadAssets component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::entity_factory::entities::global::animated::components::AnimatedComponent;

use super::components::{Animated, AnimationTimer};

#[derive(Resource, Default)]
pub struct GameTextures {
    pub player_atlas: Option<Handle<TextureAtlas>>,
    pub zombie_atlas: Option<Handle<TextureAtlas>>,
}

pub struct LoadAssetsPlugin;

impl Plugin for LoadAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTextures::default())
            .add_system(assets_animation_ticker);
    }
}

fn assets_animation_ticker(
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &AnimatedComponent,
        ),
        With<Animated>,
    >,
) {
    for (entity, mut timer, mut sprite, animate_data) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += animate_data.direction as usize; // move to next sprite cell
            if sprite.index >= animate_data.index_end as usize
                || sprite.index < animate_data.index_start as usize
            {
                sprite.index = animate_data.index_start as usize;
            }
        }
    }
}
