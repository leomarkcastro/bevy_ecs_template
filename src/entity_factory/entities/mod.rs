pub mod room;
pub mod ui;
use bevy::prelude::App;

pub mod blockv1;
pub mod blockv2;
pub mod blockv3;
pub mod global;
pub mod pickupablev1;
pub mod playerv1;
pub mod playerv2;
pub mod polygonv1;
pub mod polygonv2;
pub mod projectiles;
pub mod zombiesv1;

use global::*;

pub fn inject_entities(app: &mut App) {
    app.add_plugin(physics_movable::systems::PhysicsMovablePlugin);
    app.add_plugin(collidable::systems::CollidablePlugin);
    app.add_plugin(proximity::systems::ProximityPlugin);
    app.add_plugin(ai::systems::AiPlugin);
    app.add_plugin(despawn::systems::DespawnPlugin);

    app.add_plugin(projectiles::systems::ProjectilesPlugin);
    app.add_plugin(playerv2::systems::Playerv2Plugin);
    app.add_plugin(zombiesv1::systems::Zombiesv1Plugin);
    app.add_plugin(pickupablev1::systems::Pickupablev1Plugin);
    app.add_plugin(blockv1::systems::Blockv1Plugin);
    app.add_plugin(blockv2::systems::Blockv2Plugin);
    app.add_plugin(blockv3::systems::Blockv3Plugin);
    app.add_plugin(polygonv1::systems::Polygonv1Plugin);
    app.add_plugin(polygonv2::systems::Polygonv2Plugin);

    app.add_plugin(ui::screen::simple_text::systems::SimpleTextPlugin);
}
