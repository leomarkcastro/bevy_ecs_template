// To describe how the Boracay component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{prelude::*, utils::HashMap};
use bevy_tasks::AsyncComputeTaskPool;
use futures_lite::future;
use rayon::result;

use crate::{
    entity_factory::entities::global::physics_movable::components::PXSize,
    game_modules::map_loader::systems::{path_loader_system, PathDataResource},
    utils::{check_collide::check_pointtorect_collide_rect, globals::MAP_SCALE},
};
use rayon::prelude::*;

use super::components::{
    GraphPoint, PathFindProcess, PathFindProcessResource, PathFindQueryEvent, COLL_BOX_SIZE,
};
use pathfinding::prelude::{astar, bfs, dijkstra};

pub struct PathFindingServerPlugin;

impl Plugin for PathFindingServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PathFindQueryEvent>()
            .insert_resource(PathFindProcessResource {
                processess: HashMap::new(),
            })
            .add_system(pathfinding_query_client_system)
            .add_system(pathfinding_resolution_system);
    }
}

fn pathfinding_query_client_system(
    path_data: Res<PathDataResource>,
    mut pathfind_query: EventReader<PathFindQueryEvent>,
    mut pathfind_process: ResMut<PathFindProcessResource>,
    collidables: Query<(&PXSize, &GlobalTransform)>,
    time: Res<Time>,
) {
    // Shared global map data
    let b_points = path_data.path_data.as_ref().unwrap().points.clone();
    let b_vertices = path_data.path_data.as_ref().unwrap().vertices.clone();
    let b_collidables = collidables.iter().collect::<Vec<_>>();

    // get the box of the collidables
    let b_collidables_box = b_collidables
        .iter()
        .map(|(collider, gtransform)| {
            // get the translation, scale of globaltransform

            let min = gtransform.to_scale_rotation_translation().2.truncate() / MAP_SCALE;
            let rotation_quat = gtransform.to_scale_rotation_translation().1;

            let max = Vec2::from((collider.width, collider.height)) / (2.);
            // rotate the collider, get the rotation Vec2

            let target_asset_angle =
                rotation_quat.to_axis_angle().1 * rotation_quat.to_axis_angle().0.z;
            // convert f32 to vec2
            let rotation = Vec2::new(target_asset_angle.cos(), target_asset_angle.sin());

            let collider = max.rotate(rotation);
            (min, max)
        })
        .collect::<Vec<_>>();

    // loop over the path_query.queries
    for event in pathfind_query.iter() {
        // Skip this query if it has a task buffer, meaning, it is already being processed

        let points = b_points.clone();
        let vertices = b_vertices.clone();
        let collidable_box = b_collidables_box.clone();
        let default: Vec2 = Vec2::ZERO;

        let a = event.start;
        let b = event.goal;

        let task = AsyncComputeTaskPool::get().spawn(async move {
            // let point_locaitons = &path_data.path_data.as_ref().unwrap().points;
            // let point_length = point_locaitons.len();

            // get the first player position

            let start: GraphPoint = GraphPoint(a);
            let goal: GraphPoint = GraphPoint(b);

            let pos_s = points.get(a as usize).unwrap_or(&default);
            let pos_g = points.get(a as usize).unwrap_or(&default);

            if collidable_box.par_iter().any(|n| {
                check_pointtorect_collide_rect(
                    pos_s,
                    &Vec2::from((COLL_BOX_SIZE, COLL_BOX_SIZE)),
                    &n.0,
                    &n.1,
                ) || check_pointtorect_collide_rect(
                    pos_g,
                    &Vec2::from((COLL_BOX_SIZE, COLL_BOX_SIZE)),
                    &n.0,
                    &n.1,
                )
            }) {
                // point is inside the collidable box
                return Some((vec![], 0));
            }

            let result_b = astar(
                &start,
                |p| p.successors_weighted_collissioned(&points, &vertices, &collidable_box),
                |p| p.distance(&points, b as u32) as usize,
                |p| *p == goal,
            );

            result_b
        });

        let current_time = time.elapsed_seconds();
        // get current time after 5 seconds
        let despawn_time = current_time + 2.0;

        pathfind_process.processess.insert(
            event.id.to_string(),
            PathFindProcess {
                start: event.start.to_owned(),
                goal: event.goal.to_owned(),
                expire_at: despawn_time,
                is_expired: false,
                path: None,
                task_buffer: Some(task),
            },
        );
    }
}

fn pathfinding_resolution_system(
    mut pathfind_process: ResMut<PathFindProcessResource>,
    time: Res<Time>,
) {
    for (query_id, process) in pathfind_process.processess.iter_mut() {
        if let Some(task) = process.task_buffer.as_mut() {
            if let Some(result) = future::block_on(future::poll_once(task)) {
                process.path = Some(result.unwrap_or((vec![], 0)).0);
                process.task_buffer = None;
            } else {
                // Check if the task is expired
                if process.expire_at < time.elapsed_seconds() {
                    process.is_expired = true;
                    process.task_buffer.take().unwrap().cancel();
                    process.task_buffer = None;
                    process.path = Some(vec![]);
                }
            }
        }
    }
}
