use crate::types::{point::Point, snake::Snake};
use macroquad::{input::KeyCode, prelude::*};

pub struct SnakeController<'a> {
    snake: &'a Snake,
}

impl<'a> SnakeController<'a> {
    pub fn new(snake: &Snake) -> SnakeController {
        SnakeController { snake }
    }

    pub fn handle_input(&self) -> Point {
        let up = (0, -1);
        let down = (0, 1);
        let right = (1, 0);
        let left = (-1, 0);
        if let Some(key) = get_last_key_pressed() {
            let dir = match key {
                KeyCode::Right => right,
                KeyCode::Left => left,
                KeyCode::Down => down,
                KeyCode::Up => up,
                _ => self.snake.dir,
            };

            dir
        } else {
            self.snake.dir
        }
    }
}
