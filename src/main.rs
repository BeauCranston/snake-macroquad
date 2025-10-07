use macroquad::prelude::*;
use std::collections::LinkedList;
use types::{game_grid::*, point::*, snake::*, snake_controller::*};
use window_conf::conf;

const GRID_SIZE: u32 = 16;

pub mod types;
pub mod window_conf;
#[macroquad::main(conf)]
async fn main() {
    let mut snake: Snake = Snake::new();
    let grid: GameGrid = GameGrid::new(screen_width(), screen_height(), GRID_SIZE);
    let game_over: bool = false;
    let mut snake_controller: SnakeController = SnakeController::new(snake.dir.clone());
    let speed = 1.0;
    let mut last_update = get_time();
    loop {
        if !game_over {
            let dir = snake_controller.handle_input();
            snake.change_dir(dir);
            if get_time() - last_update > speed {
                println!("{} {} current dir", dir.0, dir.1);
                snake.update();
                last_update = get_time();
            }
            clear_background(WHITE);
            snake.draw(&grid);
            grid.draw();
        }

        //handle input

        //update logic

        //draw

        next_frame().await
    }
}
