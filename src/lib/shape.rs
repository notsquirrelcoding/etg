#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Shape {
    width: i32,
    height: i32,
}

impl Shape {
    pub fn new(width: i32, height: i32) -> Shape {
        Self { width, height }
    }
}
