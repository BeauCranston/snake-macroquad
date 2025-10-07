use ::rand::random_range;
use macroquad::{prelude::*, rand::rand};
use std::collections::LinkedList;
use types::{game_grid::*, point::*, snake::*, snake_controller::*};
use window_conf::conf;

const GRID_SIZE: u32 = 16;

pub mod types;
pub mod window_conf;

fn spawn_fruit() -> Point {
    let num = random_range(1..16);
    let num2 = random_range(1..16);

    (num, num2)
}

fn draw_fruit(grid: &GameGrid, point: Point) {
    draw_rectangle(
        grid.offset_x + point.0 as f32 * grid.square_size,
        grid.offset_y + point.1 as f32 * grid.square_size,
        grid.square_size,
        grid.square_size,
        RED,
    );
}

#[macroquad::main(conf)]
async fn main() {
    let mut snake: Snake = Snake::new();
    let grid: GameGrid = GameGrid::new(screen_width(), screen_height(), GRID_SIZE);
    let mut game_over: bool = false;
    let mut snake_controller: SnakeController = SnakeController::new(snake.dir.clone());
    let speed = 0.5;
    let food_spawn_rate = 2.0;
    let mut score = 0;
    let mut last_movement_update = get_time();
    let mut last_fruit_spawn = get_time();
    let mut current_fruit_point = (5, 5);
    loop {
        if !game_over {
            let dir = snake_controller.handle_input();
            snake.change_dir(dir);
            if get_time() - last_movement_update > speed {
                println!("{} {} current dir", dir.0, dir.1);
                snake.update();
                if !snake.check_inbounds(&grid) {
                    game_over = true;
                }
                last_movement_update = get_time();
            }
            if get_time() - last_fruit_spawn > food_spawn_rate {
                current_fruit_point = spawn_fruit();
                last_fruit_spawn = get_time();
            }
            clear_background(WHITE);
            snake.draw(&grid);
            grid.draw();
            draw_fruit(&grid, current_fruit_point);
        }

        next_frame().await
    }
}
