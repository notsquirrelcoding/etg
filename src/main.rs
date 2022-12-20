use lib::{screen::{SCREEN_WIDTH, SCREEN_HEIGHT, Screen}, Point};

mod lib;



fn main() {
    let mut screen = Screen::new();
    screen.render_pixel(Point::new(0, 0));
}