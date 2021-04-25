pub mod ascii_table;
pub mod color;
pub mod commit_window;
pub mod diff_window;
pub mod main_window;
pub mod renderer;
pub mod window;

pub trait Render {
    fn render(&mut self);
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}
