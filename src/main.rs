//! Retro Snaker
//!

pub mod bean;
pub mod config;
pub mod direction;
pub mod state;
pub mod player;
pub mod render;
pub mod snaker;
pub mod context;
pub mod collision;
pub mod hud;

use state::*;

fn main() {
    let mut context = context::setup();
    let mut state = GameState::new();

    state.r#loop(&mut context);

    println!("游戏结束，本局得分为: {} 分!", state.default_player.scores);
}
