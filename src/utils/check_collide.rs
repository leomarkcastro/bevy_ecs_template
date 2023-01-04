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

pub fn check_point_collide_rect(point: &Vec2, rect: &Vec2, size: &Vec2) -> bool {
    if point.x > rect.x
        && point.x < rect.x + size.x
        && point.y > rect.y
        && point.y < rect.y + size.y
    {
        return true;
    }
    false
}

pub fn check_pointtorect_collide_rect(
    point: &Vec2,
    point_size: &Vec2,
    rect: &Vec2,
    rect_size: &Vec2,
) -> bool {
    // make a rect where the center of the rec tis the point
    let rect = Vec2::new(rect.x - rect_size.x / 2., rect.y - rect_size.y / 2.);
    let point_rect = Vec2::new(point.x - point_size.x / 2., point.y - point_size.y / 2.);
    // println!("{} {} {} {}", point_rect, point_size, rect, rect_size);
    if point_rect.x < rect.x + rect_size.x
        && point_rect.x + point_size.x > rect.x
        && point_rect.y < rect.y + rect_size.y
        && point_rect.y + point_size.y > rect.y
    {
        // println!("{} {} {} {}", point_rect, point_size, rect, rect_size);
        return true;
    }
    false
}
