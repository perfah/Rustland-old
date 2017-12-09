pub mod program;
pub mod frame;
pub mod wallpaper;

use self::program::GraphicsProgram;

use wlc::Geometry;

pub trait Renderable {
    fn draw(&mut self, program: &GraphicsProgram, viewport: Geometry);
}
