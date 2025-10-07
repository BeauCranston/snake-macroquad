use crate::types::{point::Point, snake::Snake};
use macroquad::{input::KeyCode, prelude::*};

pub struct SnakeController {
    initial_dir: Point,
}

impl<'a> SnakeController {
    pub fn new(initial_dir: Point) -> SnakeController {
        SnakeController { initial_dir }
    }

    pub fn handle_input(&mut self) -> Point {
        let up = (0, -1);
        let down = (0, 1);
        let right = (1, 0);
        let left = (-1, 0);
        if let Some(key) = get_last_key_pressed() {
            let dir = match key {
                KeyCode::Right => {
                    println!("right key was pressed");
                    right
                }
                KeyCode::Left => left,
                KeyCode::Down => down,
                KeyCode::Up => up,
                _ => self.initial_dir,
            };

            if self.initial_dir.0 + dir.0 != 0 && self.initial_dir.1 + dir.1 != 0 {
                self.initial_dir = dir;
            }
        }
        self.initial_dir
    }
}
