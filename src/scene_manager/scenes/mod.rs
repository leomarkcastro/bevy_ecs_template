use bevy::prelude::App;

use super::manager::scene_list::GameScene;

pub mod test_scene01;
pub mod test_scene02;
pub mod test_scene03;

pub const DEFAULT_SCENE: GameScene = GameScene::Scene03;

pub fn inject_scenes(app: &mut App) {
    app.add_plugin(test_scene01::systems::Scene01Plugin);
    app.add_plugin(test_scene02::systems::Scene02Plugin);
    app.add_plugin(test_scene03::systems::Scene03Plugin);
}
