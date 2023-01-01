// To describe how the Despawn component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::utils::check_2circle_collide::{check_2circle_collide, CircleCollideData};

use super::DespawnComponent;

#[derive(Resource)]
pub struct DespawnTrackerGlobal {
    pub spawned_id: Vec<String>,
}

impl Default for DespawnTrackerGlobal {
    fn default() -> Self {
        Self { spawned_id: vec![] }
    }
}

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DespawnTrackerGlobal::default())
            .add_system(despawn_system);
    }
}

fn despawn_system(
    mut command: Commands,
    mut query: Query<(Entity, &DespawnComponent, &Transform)>,
    mut despawn_tracker: ResMut<DespawnTrackerGlobal>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    use bevy::math::Vec3Swizzles;
    // get camera xy
    let camera_xy = camera_query.single().translation.xyy().xy();

    // Multiple Queries
    for despawn_component in query.iter_mut() {
        let (entity, mut despawn_component, transform) = despawn_component;

        let despawn_xy = transform.translation.xyy().xy();

        if (!check_2circle_collide(
            CircleCollideData {
                center: camera_xy,
                radius: despawn_component.camera_circle,
            },
            CircleCollideData {
                center: despawn_xy,
                radius: despawn_component.bldg_circle,
            },
        )) {
            // despawn entity
            // println!("Despawning entity: {}", despawn_component.id);
            // remove id from tracker
            despawn_tracker
                .spawned_id
                .retain(|id| id != &despawn_component.id);
            command.entity(entity).despawn_recursive();
        }
    }
}
