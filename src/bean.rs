use piston::input::RenderArgs;

use crate::config::*;
use crate::render::*;
use crate::snaker::*;

pub struct Bean {
    pub x: u32,
    pub y: u32,
}

impl Bean {
    pub fn new() -> Bean {
        Bean { x: 1, y: 1 }
    }

    pub fn update(&mut self, s: &Snake) -> bool {
        let front = s.snake_parts.front().unwrap();
        if front.0 == self.x && front.1 == self.y {
            true
        } else {
            false
        }
    }

    pub fn render(&mut self, render_engine: &mut RenderEngine, args: &RenderArgs) {
        let x = self.x * SQUARE_WIDTH;
        let y = self.y * SQUARE_WIDTH;

        render_engine.draw_bean(x, y, args);
    }
}
