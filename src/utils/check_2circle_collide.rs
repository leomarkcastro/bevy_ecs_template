use bevy::prelude::Vec2;

pub struct CircleCollideData {
    pub center: Vec2,
    pub radius: f32,
}

pub fn check_2circle_collide(c1: CircleCollideData, c2: CircleCollideData) -> bool {
    let distance = (c1.center - c2.center).length();
    if distance < c1.radius + c2.radius {
        return true;
    }
    false
}
