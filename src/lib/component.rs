use super::{Point, shape::Shape};

pub struct Component {
    pos: Point,
    shape: Shape
}

impl Component {
    pub fn new(pos: Point, shape: Shape) -> Self { 
        Self { pos, shape }
    }

    pub fn action(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.pos.y += 1,
            Direction::Down => self.pos.y -= 1,
            Direction::Left => self.pos.x -= 1,
            Direction::Right => self.pos.x += 1,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}