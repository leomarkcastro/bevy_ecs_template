// To describe how the LoadAssets component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::components::{Animated, AnimationTimer};

pub struct LoadAssetsPlugin;

impl Plugin for LoadAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets_and_animations)
            .add_system(assets_animation_ticker);
    }
}

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub idle_gun: Handle<TextureAtlas>,
}

fn load_assets_and_animations(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = assets_server.load("image_sprite_humans/idle_gun.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(423., 349.), 8, 1, None, None);
    let explosion = texture_atlases.add(texture_atlas);

    let player = assets_server.load("image_sprite_humans/idle_gun/Idle_gun_000.png");

    let game_textures = GameTextures {
        player,
        idle_gun: explosion,
    };
    commands.insert_resource(game_textures);
}

fn assets_animation_ticker(
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnimationTimer, &mut TextureAtlasSprite), With<Animated>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1; // move to next sprite cell
            if sprite.index >= 7 {
                sprite.index = 0;
            }
        }
    }
}
