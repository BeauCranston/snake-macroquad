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
        self.dir = point;
    }
    pub fn update(&mut self) {
        let old_head = self.head.clone();
        self.head = (self.head.0 + self.dir.0, self.head.1 + self.dir.1);
        self.body.push_front(old_head);
        self.body.pop_back();
    }
    pub fn draw(&self, grid: &GameGrid) {
        //draw head
        Self::draw_snake_part(grid, &self.head, DARKGREEN);
        for point in &self.body {
            Self::draw_snake_part(grid, point, GREEN);
        }
    }
}
