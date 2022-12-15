#![allow(unused)]
#![deny(clippy::all)]

mod game_modules;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Template Bevy Project".to_string(),
                width: 640.0,
                height: 480.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .run();
}
