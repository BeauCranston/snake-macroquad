use std::collections::LinkedList;

use macroquad::{
    color::{Color, DARKGREEN, GREEN, WHITE},
    math::vec2,
    shapes::draw_rectangle,
    texture::{DrawTextureParams, Texture2D, draw_texture, draw_texture_ex, load_texture},
};

use crate::{Point, types::game_grid::GameGrid};

pub struct Snake {
    pub head: Point,
    pub head_texture: Texture2D,
    pub dir: Point,
    pub body: LinkedList<Point>,
}

impl Snake {
    pub fn new(head_texture: Texture2D) -> Snake {
        Snake {
            head: (8, 8),
            head_texture: head_texture,
            dir: (1, 0),
            body: LinkedList::new(),
        }
    }
    pub fn reset(&mut self) {
        self.head = (8, 8);
        self.body = LinkedList::new();
        self.dir = (1, 0);
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
    pub fn draw(&self, grid: &GameGrid) {
        //draw head
        // Self::draw_snake_part(grid, &self.head, DARKGREEN);
        let rotation = match self.dir.0 {
            -1 => -90,
            0 => 0,
            1 => 90,
            _ => 0,
        };
        draw_texture_ex(
            &self.head_texture,
            grid.offset_x + self.head.0 as f32 * grid.square_size,
            grid.offset_y + self.head.1 as f32 * grid.square_size,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(grid.square_size, grid.square_size)),
                rotation: rotation as f32,
                flip_y: self.dir.1 > 0,
                ..Default::default()
            },
        );
        for point in &self.body {
            Self::draw_snake_part(grid, point, GREEN);
        }
    }
}
