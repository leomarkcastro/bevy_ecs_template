use bevy::prelude::*;
use bevy_rapier2d::{prelude::RigidBody, rapier::prelude::Collider, *};

// pub fn spawn_line_segment(mut commands: Commands, p1: Vec2, p2: Vec2) {
//     let mut body = commands.spawn(SpriteBundle {
//         sprite: Sprite {
//             color: Color::rgb(0.0, 1.0, 0.0),
//             custom_size: Some(Vec2::new(1.0, 1.0)),
//             ..Default::default()
//         },
//         transform: Transform {
//             translation: Vec3::new((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0, 0.0),
//             rotation: p1.y.atan2(p1.x),
//             ..Default::default()
//         },
//         ..Default::default()
//     });
//     body.insert(Collider::line_segment(p1, p2));
//     body.insert(RigidBody::Static);
// }
