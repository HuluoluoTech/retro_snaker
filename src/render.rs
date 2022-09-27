use crate::config::OPENGL_VERSION;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use crate::config::*;
use crate::snaker::*;
use std::collections::LinkedList;
use crate::hud::*;

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

    pub fn clear(&mut self, args: &RenderArgs) {
        self.engine.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });
    }

    pub fn draw_bean(&mut self, x: u32, y: u32, args: &RenderArgs) {
        self.engine.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            let square = graphics::rectangle::square(x as f64, y as f64, SQUARE_WIDTH as f64);

            graphics::rectangle(RED, square, transform, gl)
        });
    }

    pub fn draw_snaker(&mut self, snake_parts: &LinkedList<SnakePiece>, width:u32, args: &RenderArgs) {
        self.engine.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            let squares: Vec<graphics::types::Rectangle> = snake_parts
                .iter()
                .map(|p| SnakePiece(p.0 * width, p.1 * width))
                .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, width as f64))
                .collect();

            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        });
    }

    pub fn draw_hud(&mut self, hud: &Hud, args: &RenderArgs) {
        self.engine.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            let square = graphics::rectangle::square(hud.x as f64, hud.y as f64, hud.width as f64);
            // graphics::text(RED, 5, &hud.text, &mut cache, transform, gl);

            graphics::rectangle(RED, square, transform, gl)
        });
    }
}
