#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn cross_mag(a: &Point, b: &Point) -> f32 {
        a.x * b.y - a.y * b.x
    }
}
