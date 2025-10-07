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

fn update(snake: &mut Snake, current_fruit_point: &mut Point, score: &mut i32) {
    // save current head in variable
    let old_head = snake.head.clone();
    //move snake head to new position
    snake.head = (snake.head.0 + snake.dir.0, snake.head.1 + snake.dir.1);
    //push old head to body
    snake.body.push_front(old_head);
    let eating_fruit =
        snake.head.0 == current_fruit_point.0 && snake.head.1 == current_fruit_point.1;
    if !eating_fruit {
        //pop off end of tail
        snake.body.pop_back();
    } else {
        //if fruit has been eaten, grow size by not popping tail
        *score += 1;
        current_fruit_point.0 = 0;
        current_fruit_point.1 = 0;
        println!("{} score", *score);
    }
}
fn check_self_eat(snake: &Snake) -> bool {
    let mut eating_self = false;
    for node in &snake.body {
        if snake.head.0 == node.0 && snake.head.1 == node.1 {
            eating_self = true;
        }
    }
    eating_self
}
fn check_inbounds(snake: &Snake, grid: &GameGrid) -> bool {
    return (0.0 <= snake.head.0 as f32 && grid.grid_size > snake.head.0 as f32)
        && (0.0 <= snake.head.1 as f32 && grid.grid_size > snake.head.1 as f32);
}

fn fruit_has_been_eaten(current_fruit_point: &Point) -> bool {
    current_fruit_point.0 == 0 && current_fruit_point.1 == 0
}

#[macroquad::main(conf)]
async fn main() {
    let mut snake: Snake = Snake::new();
    let grid: GameGrid = GameGrid::new(screen_width(), screen_height(), GRID_SIZE);
    let mut game_over: bool = false;
    let mut snake_controller: SnakeController = SnakeController::new(snake.dir.clone());
    let speed = 0.5;
    let food_spawn_rate = 2.0;
    let mut score = 0 as i32;
    let mut last_movement_update = get_time();
    let mut last_fruit_spawn = get_time();
    let mut current_fruit_point = (5, 5);
    loop {
        if !game_over {
            let dir = snake_controller.handle_input();
            snake.change_dir(dir);
            if get_time() - last_movement_update > speed {
                println!("{} {} current dir", dir.0, dir.1);
                update(&mut snake, &mut current_fruit_point, &mut score);
                if !check_inbounds(&snake, &grid) || check_self_eat(&snake) {
                    game_over = true;
                }
                last_movement_update = get_time();
            }
            if get_time() - last_fruit_spawn > food_spawn_rate {
                if fruit_has_been_eaten(&current_fruit_point) {
                    current_fruit_point = spawn_fruit();
                }
                last_fruit_spawn = get_time();
            }
            clear_background(WHITE);
            snake.draw(&grid);
            grid.draw();
            if !fruit_has_been_eaten(&current_fruit_point) {
                draw_fruit(&grid, current_fruit_point);
            }
        }

        next_frame().await
    }
}
