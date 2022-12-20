// To describe how the Collidable component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;
use bevy_rapier2d::prelude::CollidingEntities;

use crate::entity_factory::entities::{
    global::health::components::HealthComponent, projectiles::components::ProjectileEffect,
};

use super::components::{CollidableBody, CollissionData};

pub struct CollidablePlugin;

impl Plugin for CollidablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collidable_system);
        //     .add_system(collidable_system);
    }
}

fn collidable_init_system() {}

fn collidable_system(
    mut command: Commands,
    mut meshes: Query<(Entity, &CollissionData, &CollidingEntities)>,
    colliding_body_data: Query<(&CollissionData, &CollidingEntities)>,
    mut health_query: Query<&mut HealthComponent>,
) {
    for (entity, col_data, col_entities) in meshes.iter_mut() {
        if col_entities.len() > 0 {
            match col_data.body {
                CollidableBody::Player => {
                    for (collision_entity) in col_entities.iter() {
                        let colliding_body_data = colliding_body_data.get(collision_entity);

                        if let Ok((colliding_body_data, _)) = colliding_body_data {
                            match colliding_body_data.body {
                                CollidableBody::Bullet => match colliding_body_data.effect {
                                    Some(ProjectileEffect::Damage { amount }) => {
                                        let mut health = health_query.get_mut(entity).unwrap();

                                        println!("Damage: {}", amount);
                                        health.health -= amount;
                                        if health.health <= 0.0 {
                                            command.entity(entity).despawn();
                                            println!("Entity Killed");
                                        }
                                    }

                                    _ => {}
                                },
                                CollidableBody::Enemy => {
                                    let mut health = health_query.get_mut(entity).unwrap();

                                    println!("Damage: {}", 1.0);
                                    health.health -= 1.0;
                                    if health.health <= 0.0 {
                                        command.entity(entity).despawn();
                                        println!("Entity Killed");
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                }
                CollidableBody::Enemy => {
                    for (collision_entity) in col_entities.iter() {
                        let colliding_body_data = colliding_body_data.get(collision_entity);

                        if let Ok((colliding_body_data, _)) = colliding_body_data {
                            match colliding_body_data.body {
                                CollidableBody::Bullet => match colliding_body_data.effect {
                                    Some(ProjectileEffect::Damage { amount }) => {
                                        let mut health = health_query.get_mut(entity).unwrap();

                                        println!("Damage: {}", amount);
                                        health.health -= amount;
                                        if health.health <= 0.0 {
                                            command.entity(entity).despawn();
                                            println!("Entity Killed");
                                        }
                                    }

                                    _ => {}
                                },

                                _ => {}
                            }
                        }
                    }
                }
                CollidableBody::Bullet => {
                    if col_entities.len() > 0 {
                        command.entity(entity).despawn();
                        // println!("Bulletv1Entity collided with something");
                    }
                }
                _ => {}
            }
            // get type of colliding entity
        }
    }
}
