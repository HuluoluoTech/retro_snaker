use piston::input::RenderArgs;

use crate::config::*;
use crate::render::*;

pub struct Bean {
    pub x: u32,
    pub y: u32,
}

impl Bean {
    pub fn new() -> Bean {
        Bean { x: 1, y: 1 }
    }

    pub fn new_random(x: u32, y: u32) -> Bean {


        Bean { x, y}

    }

    pub fn render(&mut self, render_engine: &mut RenderEngine, args: &RenderArgs) {
        let x = self.x * SQUARE_WIDTH;
        let y = self.y * SQUARE_WIDTH;

        render_engine.draw_bean(x, y, args);
    }
}
