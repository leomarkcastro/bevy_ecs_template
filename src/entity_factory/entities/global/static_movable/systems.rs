// To describe how the Global component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use super::components::MovableComponent;

pub struct StaticMovable;

impl Plugin for StaticMovable {
    fn build(&self, app: &mut App) {
        app.add_system(input_movement_system);
    }
}

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

fn input_movement_system(mut query: Query<(&MovableComponent, &mut Transform)>) {
    for (input_movable, mut transform) in query.iter_mut() {
        let mut translation = &mut transform.translation;
        translation.x += input_movable.vec_x * TIME_STEP * BASE_SPEED;
        translation.y += input_movable.vec_y * TIME_STEP * BASE_SPEED;
    }
}
