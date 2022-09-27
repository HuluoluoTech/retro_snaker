use piston::input::Button;
use piston::input::Key;
use piston::input::RenderArgs;
use piston::input::UpdateArgs;

use crate::bean::*;
use crate::config::*;
use crate::direction::*;
use crate::player::*;
use crate::render::*;
use crate::snaker::*;

pub struct GameState {
    pub rows: u32,
    pub cols: u32,
    pub just_eaten: bool,

    pub bean: Bean,
    pub snake: Snake,
    pub default_player: Player,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            rows: ROWS,
            cols: COLS,
            just_eaten: false,

            bean: Bean::new(),
            snake: Snake::new(),
            default_player: Player::default(),
        }
    }

    pub fn clear(&mut self, render_engine: &mut RenderEngine, args: &RenderArgs) {
        render_engine.clear(args);
    }

    pub fn render(&mut self, render_engine: &mut RenderEngine, args: &RenderArgs) {
        self.clear(render_engine, args);

        self.snake.render(render_engine, args);
        self.bean.render(render_engine, args);
    }

    pub fn update(&mut self, _args: &UpdateArgs) -> bool {
        if !self.snake.update(self.just_eaten, self.cols, self.rows) {
            return false;
        }

        if self.just_eaten {
            self.default_player.update(1);

            self.just_eaten = false;
        }

        self.just_eaten = self.bean.update(&self.snake);
        if self.just_eaten {
            use rand::thread_rng;
            use rand::Rng;

            let mut r = thread_rng();
            loop {
                let new_x = r.gen_range(0, self.cols);
                let new_y = r.gen_range(0, self.rows);
                if !self.snake.is_collide(new_x, new_y) {
                    self.bean = Bean { x: new_x, y: new_y };
                    break;
                }
            }
        }

        true
    }

    pub fn pressed(&mut self, direction: &Button) {
        let last_direction = self.snake.d.clone();
        self.snake.d = match direction {
            &Button::Keyboard(Key::Up) if last_direction != Direction::DOWN => Direction::UP,
            &Button::Keyboard(Key::Down) if last_direction != Direction::UP => Direction::DOWN,
            &Button::Keyboard(Key::Left) if last_direction != Direction::RIGHT => Direction::LEFT,
            &Button::Keyboard(Key::Right) if last_direction != Direction::LEFT => Direction::RIGHT,
            _ => last_direction,
        };
    }
}
