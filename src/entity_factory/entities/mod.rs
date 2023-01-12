pub mod ui;
use bevy::prelude::App;

pub mod blockv1;
pub mod blockv2;
pub mod blockv3;
pub mod global;
pub mod pickupablev1;
pub mod playerv1;
pub mod playerv2;
pub mod playerv3;
pub mod polygonv1;
pub mod polygonv2;
pub mod projectiles;
pub mod roofv1;
pub mod roomv1;
pub mod treev1;
pub mod zombiesv1;

use global::*;

pub fn inject_entities(app: &mut App) {
    app.add_plugin(physics_movable::systems::PhysicsMovablePlugin);
    app.add_plugin(collidable::systems::CollidablePlugin);
    app.add_plugin(proximity::systems::ProximityPlugin);
    app.add_plugin(ai::systems::AiPlugin);
    app.add_plugin(despawn::systems::DespawnPlugin);
    app.add_plugin(despawn_on_clock::systems::DespawnWithTimerPlugin);
    app.add_plugin(dissapear_proximity::systems::DissapearProximityPlugin);

    app.add_plugin(projectiles::systems::ProjectilesPlugin);
    app.add_plugin(playerv2::systems::Playerv2Plugin);
    app.add_plugin(playerv3::systems::Playerv3Plugin);
    app.add_plugin(zombiesv1::systems::Zombiesv1Plugin);
    app.add_plugin(pickupablev1::systems::Pickupablev1Plugin);
    app.add_plugin(blockv1::systems::Blockv1Plugin);
    app.add_plugin(blockv2::systems::Blockv2Plugin);
    app.add_plugin(blockv3::systems::Blockv3Plugin);
    app.add_plugin(roofv1::systems::Roofv1Plugin);
    app.add_plugin(polygonv1::systems::Polygonv1Plugin);
    app.add_plugin(polygonv2::systems::Polygonv2Plugin);
    app.add_plugin(roomv1::systems::RoomV1Plugin);
    app.add_plugin(treev1::systems::Treev1Plugin);

    app.add_plugin(ui::screen::simple_text::systems::SimpleTextPlugin);
}
