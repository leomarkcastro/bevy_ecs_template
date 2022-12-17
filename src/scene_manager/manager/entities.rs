use bevy::prelude::Component;

#[derive(Debug)]
pub enum SpawnAt {
    World01,
    UI,
}

// Base World
#[derive(Component)]
pub struct World01;

#[derive(Component)]
pub struct UI;
