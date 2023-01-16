use bevy::prelude::App;

use super::manager::scene_list::GameScene;

pub mod test_scene01;
pub mod test_scene02;
pub mod test_scene03;
pub mod test_scene04;
pub mod test_scene05;
pub mod test_scene06;

pub const DEFAULT_SCENE: GameScene = GameScene::Scene06;

pub fn inject_scenes(app: &mut App) {
    app.add_plugin(test_scene01::systems::Scene01Plugin);
    app.add_plugin(test_scene02::systems::Scene02Plugin);
    app.add_plugin(test_scene03::systems::Scene03Plugin);
    app.add_plugin(test_scene04::systems::Scene04Plugin);
    app.add_plugin(test_scene05::systems::Scene05Plugin);
    app.add_plugin(test_scene06::systems::Scene06Plugin);
}
