// To describe how the Ai component/entity should behave.
// WILL: contain pure logic that interacts with the component

use std::ops::ControlFlow;

use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{
    entity_factory::entities::global::physics_movable::components::PXMovableComponent,
    game_modules::timers::components::{OneSecondTimer, ThreeSecondTimer},
};

use super::{
    components::{AIDetectionData, AIIdentifier, AIMode, AIStatus, AITeam},
    entities::AIEntity,
};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(ai_init_system).add_system(ai_system);
        app.add_system(ai_action_system)
            .add_system(ai_detection_system);
    }
}

fn ai_action_system(
    time: Res<Time>,
    mut three_sec_timer: ResMut<ThreeSecondTimer>,
    mut commands: Commands,
    mut ai_query: Query<(Entity, &mut AIStatus, &GlobalTransform), With<AIEntity>>,
    ai_identifier_query: Query<(&AIIdentifier, &GlobalTransform), With<AIEntity>>,
    mut physics_query: Query<(&mut PXMovableComponent, &GlobalTransform), With<AIEntity>>,
) {
    for (entity, mut ai_status, &current_ai_gtransform) in ai_query.iter_mut() {
        if (!ai_status.active) {
            continue;
        }
        let current_location = current_ai_gtransform.to_scale_rotation_translation().2;
        match ai_status.mode {
            AIMode::Idle if three_sec_timer.event_timer.tick(time.delta()).finished() => {
                if let ControlFlow::Break(_) =
                    action_idle_mode(&mut ai_status, &current_ai_gtransform)
                {
                    continue;
                }
            }
            AIMode::Patrol { duration, path } => {
                if let ControlFlow::Break(_) = action_patrol_mode(
                    &mut physics_query,
                    entity,
                    duration,
                    &time,
                    path,
                    &current_ai_gtransform,
                    ai_status,
                ) {
                    continue;
                }
            }
            AIMode::Attack { target } => {
                // query for target based on AIStatus.id
                let query = ai_identifier_query
                    .iter()
                    .find(|(ai_status, _)| ai_status.id == target);

                if query.is_none() {
                    ai_status.mode = AIMode::Idle;
                    continue;
                }

                let (_, target_transform) = query.unwrap();

                let path = target_transform.to_scale_rotation_translation().2;

                let mut pxmovable = physics_query.get_component_mut::<PXMovableComponent>(entity);

                // println!("Patrol");
                // decrease duration
                if let Ok(mut pxmovable) = pxmovable {
                    // get distance from current position to target position
                    let distance_x = path.x - current_location.x;
                    let distance_y = path.y - current_location.y;

                    let base_move_speed = 0.8;

                    let move_speed = |start: f32, end: f32| -> f32 {
                        // lerp between start and end
                        let distance = end - start;
                        let lerp_speed = distance * 0.1;

                        // get the min of the lerp speed and the base move speed
                        let move_speed = lerp_speed.abs().min(base_move_speed);

                        // if the distance is negative, then the move speed should be negative
                        if distance < 0.0 {
                            -move_speed
                        } else {
                            move_speed
                        }
                    };

                    pxmovable.vec_x = move_speed(current_location.x, path.x);
                    pxmovable.vec_y = move_speed(current_location.y, path.y);
                    pxmovable.angle = pxmovable.vec_y.atan2(pxmovable.vec_x);
                }
            }
            AIMode::Flee => {
                // TODO: Change into something else
                ai_status.mode = AIMode::Idle;
            }
            _ => {}
        }
    }
}

fn action_patrol_mode(
    physics_query: &mut Query<(&mut PXMovableComponent, &GlobalTransform), With<AIEntity>>,
    entity: Entity,
    duration: f32,
    time: &Res<Time>,
    path: Vec2,
    transform: &GlobalTransform,
    mut ai_status: Mut<AIStatus>,
) -> ControlFlow<()> {
    let mut pxmovable = physics_query.get_component_mut::<PXMovableComponent>(entity);
    let duration = duration - time.delta_seconds();
    // println!("Patrol");
    // decrease duration
    if let Ok(mut pxmovable) = pxmovable {
        let current_location = transform.to_scale_rotation_translation().2;
        // get distance from current position to target position
        let distance_x = path.x - current_location.x;
        let distance_y = path.y - current_location.y;

        // check if current position is close to target position or if time duration has passed
        if (current_location.xy().distance(path) < 1.5) || (duration <= 0.0) {
            ai_status.mode = AIMode::Idle;
            pxmovable.vec_x = 0.0;
            pxmovable.vec_y = 0.0;
            // println!("Patrol has reached target");
            return ControlFlow::Break(());
        }

        let base_move_speed = 0.5;

        let move_speed = |start: f32, end: f32| -> f32 {
            // lerp between start and end
            let distance = end - start;
            let lerp_speed = distance * 0.1;

            // get the min of the lerp speed and the base move speed
            let move_speed = lerp_speed.abs().min(base_move_speed);

            // if the distance is negative, then the move speed should be negative
            if distance < 0.0 {
                -move_speed
            } else {
                move_speed
            }
        };

        pxmovable.vec_x = move_speed(current_location.x, path.x);
        pxmovable.vec_y = move_speed(current_location.y, path.y);
        pxmovable.angle = pxmovable.vec_y.atan2(pxmovable.vec_x);
    }
    ControlFlow::Continue(())
}

fn action_idle_mode(
    ai_status: &mut Mut<AIStatus>,
    current_ai_gtransform: &GlobalTransform,
) -> ControlFlow<()> {
    if (!ai_status.can_move) {
        return ControlFlow::Break(());
    }
    let start = current_ai_gtransform
        .to_scale_rotation_translation()
        .2
        .truncate();
    let mut rand_x = rand::random::<f32>() * -10.0 + 5.0;
    let mut rand_y = rand::random::<f32>() * -10.0 + 5.0;
    rand_x = rand_x * 5.;
    rand_y = rand_y * 5.;
    let end = start + Vec2::new(rand_x, rand_y);
    ai_status.mode = AIMode::Patrol {
        duration: 3.0,
        path: end,
    };
    // randomize if should patrol or not
    // let rand = rand::random::<f32>() * 100.0;
    // if rand > 20.0 {
    //     ai_status.mode = AIMode::Idle;
    //     continue;
    // }
    // get location to walk to
    // get random x and y between -5 and 5
    // println!("Patrol {:?} to {:?}", start, end);
    ControlFlow::Continue(())
}

fn ai_detection_system(
    mut ai_query: Query<(&AIIdentifier, &mut AIStatus, &GlobalTransform), With<AIEntity>>,
    ai_b_query: Query<(&AIIdentifier, &AIDetectionData, &GlobalTransform), With<AIEntity>>,
) {
    for (identifier, mut ai_status, &ai_gtransform) in ai_query.iter_mut() {
        if ai_status.active == false {
            continue;
        }

        let start = ai_gtransform.to_scale_rotation_translation().2.truncate();

        match identifier.team {
            AITeam::Zombies => {
                // query all players
                let mut player_filter = ai_b_query.iter().filter(|(identifier, _, _)| {
                    if let AITeam::Player = identifier.team {
                        return true;
                    }
                    false
                });

                // return; // TODO: THIS IS A TEMPORARY RETURN STATEMENT

                // check if any player is within detection range
                for (identifier, detection, player_gtransform) in player_filter {
                    let player_location = player_gtransform
                        .to_scale_rotation_translation()
                        .2
                        .truncate();
                    let distance = start.distance(player_location);
                    if distance < detection.detection_radius {
                        ai_status.mode = AIMode::Attack {
                            target: identifier.id,
                        };
                        // println!("Zombie has detected player");
                        break;
                    } else if let AIMode::Attack { target } = ai_status.mode {
                        if target == identifier.id {
                            ai_status.mode = AIMode::Idle;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
