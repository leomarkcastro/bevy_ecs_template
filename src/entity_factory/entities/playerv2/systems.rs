// To describe how the Playerv1 component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};

use crate::{
    entity_factory::{
        entities::global::{
            physics_movable::{components::PXMovableComponent, systems::insert_physics_components},
            static_movable::components::MovableComponent,
        },
        factory::data::SpawnEntityEvent,
    },
    game_modules::controllable::components::ControllableComponent,
};

use super::{components::InputBind, Playerv2Entity};

pub struct Playerv2Plugin;

impl Plugin for Playerv2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(playerv2_control_system);
    }
}

pub fn plaverv2_spawn(mut commands: &mut Commands, spawn_entity_event: &SpawnEntityEvent) {
    let mut body = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: spawn_entity_event.position.unwrap_or_default(),
            rotation: spawn_entity_event.rotation.unwrap_or_default(),
            ..Default::default()
        },
        ..Default::default()
    });

    body.insert(Playerv2Entity)
        .insert(InputBind { active: true });
    insert_physics_components(&mut body);
}

fn playerv2_control_system(
    controllable_query: Query<&ControllableComponent>,
    mut query: Query<(&InputBind, &mut PXMovableComponent), With<Playerv2Entity>>,
) {
    let controller = controllable_query.single();
    for (input_bind, mut movable) in query.iter_mut() {
        if (!input_bind.active) {
            return;
        }
        movable.vec_x = controller.joy_x;
        movable.vec_y = controller.joy_y;
    }
}
