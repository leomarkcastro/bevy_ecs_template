use crate::{
    game_modules::map_loader::data::PathData,
    utils::check_collide::{check_point_collide_rect, check_pointtorect_collide_rect},
};
use bevy::{
    prelude::{Component, Resource, Transform, Vec2},
    utils::{HashMap, Uuid},
};
use bevy_rapier2d::prelude::Collider;
use bevy_tasks::Task;
use rayon::prelude::*;
use std::sync::mpsc::channel;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GraphPoint(pub u32);

pub const COLL_BOX_SIZE: f32 = 15.0;

impl GraphPoint {
    pub fn successors(&self, path_data: &PathData) -> Vec<GraphPoint> {
        let (sender, receiver) = channel();

        let &GraphPoint(index) = self;

        // get the vertices index from the vertices dictionary
        path_data
            .vertices
            .get(index as usize)
            .unwrap_or(&vec![])
            .into_par_iter()
            .for_each_with(sender, |s, x| s.send(GraphPoint(*x)).unwrap());

        let mut res: Vec<GraphPoint> = receiver.iter().collect();

        res

        // convert the vertices into points
    }

    pub fn successors_weighted(&self, path_data: &PathData) -> Vec<(GraphPoint, usize)> {
        let (sender, receiver) = channel();

        let &GraphPoint(index) = self;

        // get the vertices index from the vertices dictionary
        path_data
            .vertices
            .get(index as usize)
            .unwrap_or(&vec![])
            .into_par_iter()
            .for_each_with(sender, |s, x| s.send((GraphPoint(*x), 1)).unwrap());

        let mut res: Vec<(GraphPoint, usize)> = receiver.iter().collect();

        res

        // convert the vertices into points
    }

    pub fn successors_weighted_collissioned(
        &self,
        points: &Vec<Vec2>,
        vertices: &Vec<Vec<u32>>,
        collission_boxes: &Vec<(Vec2, Vec2)>,
    ) -> Vec<(GraphPoint, usize)> {
        let (sender, receiver) = channel();

        let &GraphPoint(index) = self;
        let default = Vec2::new(0.0, 0.0);

        // get the vertices index from the vertices dictionary
        vertices
            .get(index as usize)
            .unwrap_or(&vec![])
            .into_par_iter()
            .for_each_with(sender, |s, x| {
                let pos = points.get(*x as usize).unwrap_or(&default);
                // for each vertex, check if it collides with any of the collission boxes
                if collission_boxes.par_iter().any(|n| {
                    check_pointtorect_collide_rect(
                        pos,
                        &Vec2::from((COLL_BOX_SIZE, COLL_BOX_SIZE)),
                        &n.0,
                        &n.1,
                    )
                }) {
                    // println!("Collission detected");
                } else {
                    s.send((GraphPoint(*x), 1)).unwrap()
                }
            });

        let mut res: Vec<(GraphPoint, usize)> = receiver.iter().collect();

        res

        // convert the vertices into points
    }

    pub fn distance(&self, points: &Vec<Vec2>, goal: u32) -> u32 {
        let &GraphPoint(x) = self;
        let default = Vec2::new(0.0, 0.0);
        let current_pos = points.get(x as usize).unwrap_or(&default);
        let goal_pos = points.get(goal as usize).unwrap_or(&default);
        current_pos.distance(*goal_pos) as u32
    }
}

pub struct PathFindQueryEvent {
    pub start: u32,
    pub goal: u32,
    pub id: Uuid,
}

impl Default for PathFindQueryEvent {
    fn default() -> Self {
        Self {
            // Input
            start: 0, // IMPORTANT: Overwrite!
            goal: 0,  // IMPORTANT: Overwrite!
            id: Uuid::new_v4(),
        }
    }
}

#[derive(Debug)]
pub struct PathFindProcess {
    pub start: u32,
    pub goal: u32,
    pub task_buffer: Option<Task<Option<(std::vec::Vec<GraphPoint>, usize)>>>,
    pub path: Option<Vec<GraphPoint>>,
    pub expire_at: f32,
    pub is_expired: bool,
}

#[derive(Resource)]
pub struct PathFindProcessResource {
    pub processess: HashMap<String, PathFindProcess>,
}
