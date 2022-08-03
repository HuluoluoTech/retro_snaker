//! Retro Snaker
//!
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::collections::LinkedList;

// Config Constants
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;
const COLS: u32 = 30;
const ROWS: u32 = 20;
const SQUARE_WIDTH: u32 = 20;
const WIDTH: u32 = COLS * SQUARE_WIDTH;
const HEIGHT: u32 = ROWS * SQUARE_WIDTH;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct GameState {
    rows: u32,
    cols: u32,
    just_eaten: bool,

    food: Food,
    snake: Snake,
    default_player: Player,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            rows: ROWS,
            cols: COLS,
            just_eaten: false,

            food: Food::new(),
            snake: Snake::new(),
            default_player: Player::default(),
        }
    }

    fn clear(&mut self, render_engine: &mut RenderEngine, args: &RenderArgs) {
        render_engine.engine.draw(args.viewport(), |c, gl| {
            graphics::clear(GREEN, gl);
        });
    }

    fn render(&mut self, render_engine: &mut RenderEngine, args: &RenderArgs) {
        self.clear(render_engine, args);
        self.snake.render(render_engine, args);
        self.food.render(render_engine, args);
    }

    fn update(&mut self, args: &UpdateArgs) -> bool {
        if !self.snake.update(self.just_eaten, self.cols, self.rows) {
            return false;
        }

        if self.just_eaten {
            self.default_player.update(1);

            self.just_eaten = false;
        }

        self.just_eaten = self.food.update(&self.snake);
        if self.just_eaten {
            use rand::thread_rng;
            use rand::Rng;
            // try my luck
            let mut r = thread_rng();
            loop {
                let new_x = r.gen_range(0, self.cols);
                let new_y = r.gen_range(0, self.rows);
                if !self.snake.is_collide(new_x, new_y) {
                    self.food = Food { x: new_x, y: new_y };
                    break;
                }
            }
        }

        true
    }

    fn pressed(&mut self, direction: &Button) {
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

#[derive(Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Snake {
    snake_parts: LinkedList<SnakePiece>,
    width: u32,
    d: Direction,
}

#[derive(Clone)]
pub struct SnakePiece(u32, u32);

impl Snake {
    pub fn new() -> Snake {
        Snake {
            snake_parts: LinkedList::from_iter((vec![SnakePiece(COLS / 2, ROWS / 2)]).into_iter()),
            width: SQUARE_WIDTH,
            d: Direction::DOWN,
        }
    }

    pub fn render(&mut self, render_engine: &mut RenderEngine, args: &RenderArgs) {
        let squares: Vec<graphics::types::Rectangle> = self
            .snake_parts
            .iter()
            .map(|p| SnakePiece(p.0 * self.width, p.1 * self.width))
            .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
            .collect();

        render_engine.engine.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

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

    fn is_collide(&self, x: u32, y: u32) -> bool {
        self.snake_parts.iter().any(|p| x == p.0 && y == p.1)
    }
}

pub struct Food {
    x: u32,
    y: u32,
}

impl Food {
    pub fn new() -> Food {
        Food { x: 1, y: 1 }
    }

    fn update(&mut self, s: &Snake) -> bool {
        let front = s.snake_parts.front().unwrap();
        if front.0 == self.x && front.1 == self.y {
            true
        } else {
            false
        }
    }

    fn render(&mut self, render_engine: &mut RenderEngine, args: &RenderArgs) {
        let x = self.x * SQUARE_WIDTH;
        let y = self.y * SQUARE_WIDTH;

        let square = graphics::rectangle::square(x as f64, y as f64, SQUARE_WIDTH as f64);

        render_engine.engine.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(BLACK, square, transform, gl)
        });
    }
}

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

#[derive(Debug, Default)]
struct Player {
    pub scores: u32,
}
impl Player {
    pub fn update(&mut self, score: u32) {
        self.scores += score;
    }
}

fn setup_context() -> GlutinWindow {
    match WindowSettings::new("RetroSnaker", [WIDTH, HEIGHT])
        .opengl(OPENGL_VERSION)
        .exit_on_esc(true)
        .build()
    {
        Ok(w) => w,
        Err(s) => panic!("Init Window Error: {}", s),
    }
}

fn r#loop(game: &mut GameState, window: &mut GlutinWindow) {
    let mut events = Events::new(EventSettings::new()).ups(10);
    let mut render_engine = RenderEngine::new();
    while let Some(e) = events.next(window) {
        if let Some(r) = e.render_args() {
            game.render(&mut render_engine, &r);
        }

        if let Some(u) = e.update_args() {
            if !game.update(&u) {
                break;
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}

fn main() {
    let mut context = setup_context();
    let mut state = GameState::new();

    r#loop(&mut state, &mut context);

    println!("游戏结束，本局得分为: {} 分!", state.default_player.scores);
}
