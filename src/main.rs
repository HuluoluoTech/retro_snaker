//! Retro Snaker
//!
use glutin_window::GlutinWindow;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub mod bean;
pub mod config;
pub mod direction;
pub mod game_state;
pub mod player;
pub mod render_engine;
pub mod snaker;

use config::*;
use game_state::*;
use render_engine::*;

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
