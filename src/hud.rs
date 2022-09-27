use crate::render::*;
use piston::input::RenderArgs;
use crate::config::*;

pub struct Hud {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub text: String,
}

impl Hud {
    pub fn new() -> Self {
        Hud {
            x: 0u32,
            y: 0u32,
            width: SQUARE_WIDTH,
            text: "0".to_owned(),
        }
    }

    pub fn update(&mut self, scores: u32) {
        self.text = scores.to_string();
    }

    pub fn render(&mut self, render_engine: &mut RenderEngine, args: &RenderArgs) {
        render_engine.draw_hud(&self, args);
    }
}