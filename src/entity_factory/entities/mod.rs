pub mod ui;
use bevy::prelude::App;

pub mod global;
pub mod playerv1;
pub mod playerv2;
pub mod projectiles;
pub mod zombiesv1;

use global::*;

pub fn inject_entities(app: &mut App) {
    // app.add_plugin(global::static_movable::systems::StaticMovable);
    app.add_plugin(global::physics_movable::systems::PhysicsMovablePlugin);
    app.add_plugin(global::collidable::systems::CollidablePlugin);
    app.add_plugin(global::ai::systems::AiPlugin);
    app.add_plugin(projectiles::systems::ProjectilesPlugin);
    // app.add_plugin(playerv1::systems::Playerv1Plugin);
    app.add_plugin(playerv2::systems::Playerv2Plugin);
    app.add_plugin(zombiesv1::systems::Zombiesv1Plugin);
    app.add_plugin(ui::screen::simple_text::systems::SimpleTextPlugin);
}
