pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

pub fn div(a: i32, b: i32) -> i32 {
    a / b
}

pub fn square(a: i32) -> i32 {
    a * a
}

pub fn sqrt(a: i32) -> f32 {
    return (a as f32).sqrt();
}

pub fn abs(a: i32) -> i32 {
    println!("abs: a: {}", a);
    if a < 0 {
        return a * -1;
    } 
    return a
}