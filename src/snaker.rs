use piston::input::RenderArgs;
use std::collections::LinkedList;

use crate::config::*;
use crate::direction::*;
use crate::render_engine::*;

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
        render_engine.engine.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            let squares: Vec<graphics::types::Rectangle> = self
                .snake_parts
                .iter()
                .map(|p| SnakePiece(p.0 * self.width, p.1 * self.width))
                .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
                .collect();

            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        })
    }

    pub fn update(&mut self, just_eaten: bool, cols: u32, rows: u32) -> bool {
        let mut new_front: SnakePiece =
            (*self.snake_parts.front().expect("No front of snake found.")).clone();

        if (self.d == Direction::UP && new_front.1 == 0)
            || (self.d == Direction::LEFT && new_front.0 == 0)
            || (self.d == Direction::DOWN && new_front.1 == rows - 1)
            || (self.d == Direction::RIGHT && new_front.0 == cols - 1)
        {
            return false;
        }

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
        if self.is_collide(new_front.0, new_front.1) {
            return false;
        }

        self.snake_parts.push_front(new_front);
        true
    }

    pub fn is_collide(&self, x: u32, y: u32) -> bool {
        self.snake_parts.iter().any(|p| x == p.0 && y == p.1)
    }
}
