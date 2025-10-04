use std::collections::LinkedList;

use macroquad::{
    color::{Color, DARKGREEN, GREEN},
    shapes::draw_rectangle,
};

use crate::{Point, types::game_grid::GameGrid};

pub struct Snake {
    pub head: Point,
    pub dir: Point,
    pub body: LinkedList<Point>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            head: (8, 8),
            dir: (1, 0),
            body: LinkedList::new(),
        }
    }
    fn draw_snake_part(grid: &GameGrid, point: &Point, color: Color) {
        draw_rectangle(
            grid.offset_x + point.0 as f32 * grid.square_size,
            grid.offset_y + point.1 as f32 * grid.square_size,
            grid.square_size,
            grid.square_size,
            color,
        );
    }
    pub fn change_dir(&mut self, point: Point) {
        if self.dir.0 + point.0 != 0 && self.dir.1 + point.1 != 0 {
            self.dir = point;
        }
    }
    pub fn draw(&self, grid: &GameGrid) {
        //draw head
        Self::draw_snake_part(grid, &self.head, DARKGREEN);
        for point in &self.body {
            Self::draw_snake_part(grid, point, GREEN);
        }
    }
}
