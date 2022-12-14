use piston::input::*;
use piston::event_loop::*;
use glutin_window::GlutinWindow;

use crate::bean::*;
use crate::config::*;
use crate::direction::*;
use crate::player::*;
use crate::render::*;
use crate::snaker::*;
use crate::collision;
use rand::thread_rng;
use rand::Rng;
use crate::hud::*;

pub struct GameState {
    pub rows: u32,
    pub cols: u32,
    pub just_eaten: bool,

    pub bean: Bean,
    pub snake: Snake,
    pub default_player: Player,
    pub hud: Hud,
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
            hud: Hud::new(),
        }
    }

    pub fn clear(&mut self, render_engine: &mut RenderEngine, args: &RenderArgs) {
        render_engine.clear(args);
    }

    pub fn render(&mut self, render_engine: &mut RenderEngine, args: &RenderArgs) {
        self.clear(render_engine, args);

        self.snake.render(render_engine, args);
        self.bean.render(render_engine, args);

        self.hud.render(render_engine, args);
    }

    pub fn update(&mut self, _args: &UpdateArgs) -> bool {
        self.just_eaten = collision::can_eat_bean(&self.snake, &self.bean);
        if self.just_eaten {
            self.new_random_bean();
        }

        if !self.snake.update(self.just_eaten, self.cols, self.rows) {
            println!("Game Over!");
            return false;
        }

        if self.just_eaten {
            self.default_player.add_score(1);
            self.just_eaten = false;
        }

        self.hud.update(self.default_player.scores);

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

    pub fn r#loop(&mut self, window: &mut GlutinWindow) {
        let mut events = Events::new(EventSettings::new()).ups(3);
        let mut render_engine = RenderEngine::new();
        while let Some(e) = events.next(window) {
            if let Some(k) = e.button_args() {
                if k.state == ButtonState::Press {
                    self.pressed(&k.button);
                }
            }

            if let Some(u) = e.update_args() {
                if !self.update(&u) {
                    break;
                }
            }
    
            if let Some(r) = e.render_args() {
                self.render(&mut render_engine, &r);
            }
        }
    }

    fn new_random_bean(&mut self) {
        let mut r = thread_rng();
        loop {
            let new_x = r.gen_range(0, COLS);
            let new_y = r.gen_range(0, ROWS);
            if !collision::is_snaker_self_colli(&self.snake, new_x, new_y) {
                self.bean = Bean { x: new_x, y: new_y };
                break;
            }
        }
    }

}
