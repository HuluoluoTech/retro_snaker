use opengl_graphics::GlGraphics;
use crate::config::OPENGL_VERSION;

trait Render {
    type Engine;

    fn render(&mut self, engine: &Self::Engine);
}

pub struct RenderEngine {
    pub engine: GlGraphics,
}

impl RenderEngine {
    pub fn new() -> RenderEngine {
        let engine = GlGraphics::new(OPENGL_VERSION);
        RenderEngine { engine }
    }
}
