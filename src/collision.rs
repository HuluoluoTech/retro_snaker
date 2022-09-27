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