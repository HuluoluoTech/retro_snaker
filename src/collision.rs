use crate::bean::*;
use crate::snaker::*;

pub fn can_eat_bean(snake: &Snake, bean: &Bean) -> bool {
    let front = snake.snake_parts.front().unwrap();
    if front.0 == bean.x && front.1 == bean.y {
        true
    } else {
        false
    }
}

pub fn is_snaker_self_colli(snake: &Snake, x: u32, y: u32) -> bool {
    snake.snake_parts.iter().any(|p| x == p.0 && y == p.1)
}