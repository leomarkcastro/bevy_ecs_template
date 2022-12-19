// To describe how the FaceAxis component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::{math::Vec3Swizzles, prelude::*};

use super::{FaceAxisEntity, FaceAxisResource};

pub struct FaceAxisPlugin;

impl Plugin for FaceAxisPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FaceAxisResource::default())
            .add_startup_system(face_axis_init_system)
            .add_system(face_axis_system);
    }
}

fn face_axis_init_system(mut commands: Commands) {}

fn face_axis_system(
    mut controllable_component: ResMut<FaceAxisResource>,
    kb: Res<Input<KeyCode>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(_position) = window.cursor_position() {
        let center_point = Vec2::new(window.width() / 2.0, window.height() / 2.0);

        // get angle between the two, the x and y distance between the two
        let x = _position.x - center_point.x;
        let y = _position.y - center_point.y;

        // get the angle between the two
        let angle = y.atan2(x);

        // set the angle
        controllable_component.angle = angle;
        controllable_component.x = x;
        controllable_component.y = y;

        // println!("Face Direction: {:?}", controllable_component);
    }
}
