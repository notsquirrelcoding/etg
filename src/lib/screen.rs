use crate::lib::component::Component;

use super::{shape::Shape, Point};

pub const SCREEN_WIDTH: i32 = 100;
pub const SCREEN_HEIGHT: i32 = 20;

pub struct Screen {
    p1: Component,
    p2: Component,
    ball: Component,
}

impl Screen {
    pub fn new() -> Screen {
        let p1_pos = Point::new(-40, 0);
        let p2_pos = Point::new(SCREEN_WIDTH - 10, 0);
        let ball_pos = Point::new(0, 0);

        let player_shape = Shape::new(2, 6);
        let ball_shape = Shape::new(2, 2);

        Screen {
            p1: Component::new(p1_pos, player_shape.clone()),
            p2: Component::new(p2_pos, player_shape),
            ball: Component::new(ball_pos, ball_shape),
        }
    }

    pub fn render_pixel(&mut self, p: Point) {
        let rows = (SCREEN_HEIGHT / 2) - p.y;
        let cols = (SCREEN_WIDTH / 2) - p.x;

        
        // Print empty rows
        (0..rows).for_each(|_| println!());
        
        // Print empty columns
        (0..cols).for_each(|_| print!(" "));
        
        // Print remaining empty rows
        print!("#");
        (0..((SCREEN_HEIGHT) - rows)).for_each(|_| println!());

    }
}
