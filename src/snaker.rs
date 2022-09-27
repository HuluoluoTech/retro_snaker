use piston::input::RenderArgs;
use std::collections::LinkedList;

use crate::config::*;
use crate::direction::*;
use crate::render::*;
use crate::collision;

pub struct Snake {
    pub snake_parts: LinkedList<SnakePiece>,
    pub width: u32,
    pub d: Direction,
}

#[derive(Clone)]
pub struct SnakePiece(pub u32, pub u32);

impl Snake {
    pub fn new() -> Snake {
        Snake {
            snake_parts: LinkedList::from_iter((vec![SnakePiece(COLS / 2, ROWS / 2)]).into_iter()),
            width: SQUARE_WIDTH,
            d: Direction::DOWN,
        }
    }

    pub fn render(&mut self, render_engine: &mut RenderEngine, args: &RenderArgs) {
        render_engine.draw_snaker(&self.snake_parts, self.width, args);
    }

    pub fn update(&mut self, just_eaten: bool, cols: u32, rows: u32) -> bool {
        let mut new_front: SnakePiece =
            (*self.snake_parts.front().expect("Empty!.")).clone();
        
        // Check edges
        if (self.d == Direction::UP && new_front.1 == 0)
            || (self.d == Direction::LEFT && new_front.0 == 0)
            || (self.d == Direction::DOWN && new_front.1 == rows - 1)
            || (self.d == Direction::RIGHT && new_front.0 == cols - 1)
        {
            return false;
        }

        // Update New front Postion
        match self.d {
            Direction::UP => new_front.1 -= 1,
            Direction::DOWN => new_front.1 += 1,
            Direction::LEFT => new_front.0 -= 1,
            Direction::RIGHT => new_front.0 += 1,
        }

        if !just_eaten {
            self.snake_parts.pop_back();
        }

        // Checks self collision.
        if collision::is_snaker_self_colli(&self, new_front.0, new_front.1) {
            return false;
        }

        self.snake_parts.push_front(new_front);

        true
    }
}
