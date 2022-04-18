use super::structure;
#[path = "../calculate/mod.rs"]
mod calculate;
pub fn nearest_distance(pointer1: structure::Point, pointer2: structure::Point) -> f32 {
    let mut x1  = pointer1.x;
    let mut y1  = pointer1.y;
    println!("x1: {}, y1: {}", x1, y1);
    let mut x2 = pointer2.x;
    let mut y2 = pointer2.y;
    println!("x2: {}, y2: {}", x2, y2);
    let x_distance = calculate::math::square(calculate::math::abs(distance_between_points(x1, x2)));
    println!("x_distance: {}", x_distance);
    let y_distance = calculate::math::square(calculate::math::abs(distance_between_points(y1, y2)));
    println!("y_distance: {}", y_distance);
    let distance = calculate::math::add(x_distance, y_distance);
    println!("distance: {}", distance);
    return calculate::math::sqrt(distance);
}

pub fn distance_between_points(point1:i32, point2:i32) -> i32{
    println!("distance_between_points : point1: {}, point2: {}", point1, point2);
    return calculate::math::sub(point1, point2);
}

pub fn hypoteneous(x1:i32, x2:i32) -> f32 {
    return calculate::math::sqrt(calculate::math::add(calculate::math::square(x1), calculate::math::square(x2)));
}