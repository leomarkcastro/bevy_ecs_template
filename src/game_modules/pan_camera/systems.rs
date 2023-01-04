// To describe how the PanCamera component/entity should behave.
// WILL: contain pure logic that interacts with the component

use std::u8::MAX;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use super::PanOrbitCamera;

pub struct PanCameraPlugin;

impl Plugin for PanCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pan_orbit_camera);
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(
        &mut PanOrbitCamera,
        &mut Transform,
        &mut OrthographicProjection,
    )>,
) {
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    // if input_mouse.pressed(orbit_button) {
    //     for ev in ev_motion.iter() {
    //         rotation_move += ev.delta;
    //     }
    // } else
    if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    // move camera with mouse
    for (mut pan_orbit_camera, mut transform, mut projection) in query.iter_mut() {
        let window_size = get_primary_window_size(&windows);
        let window_size = Vec2::new(window_size.x as f32, window_size.y as f32);
        let window_size = Vec2::new(window_size.x / -2.0, window_size.y / 2.0);

        let pan = pan / window_size * 50.0 * projection.scale;
        let pan = Vec3::new(pan.x, pan.y, 0.0);

        transform.translation += pan;
        //clamp scale to 0.1 - 10.0
        projection.scale = projection.scale - scroll * 1.0;
        projection.scale = projection.scale.clamp(0.1, 50.0);
    }
}
