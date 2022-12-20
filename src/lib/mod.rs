pub mod screen;
mod component;
mod shape;

trait Render {
     fn render(&mut self) -> Result<(), RenderError>;
}

#[derive(thiserror::Error, Debug)]
pub enum RenderError {
    #[error("Component failed to render")]
    Failed
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x, y
        }
    }    
}