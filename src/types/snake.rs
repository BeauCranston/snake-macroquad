use std::{collections::LinkedList, f32::consts::PI};

use macroquad::{
    color::{Color, DARKGREEN, GREEN, WHITE},
    math::{self, vec2},
    shapes::draw_rectangle,
    texture::{DrawTextureParams, Texture2D, draw_texture, draw_texture_ex, load_texture},
};

use crate::{Point, types::game_grid::GameGrid};

pub struct Snake {
    pub head: Point,
    pub head_texture: Texture2D,
    pub body_texture: Texture2D,
    pub dir: Point,
    pub body: LinkedList<Point>,
    pub dir_change_points: LinkedList<Point>,
}

impl Snake {
    pub fn new(head_texture: Texture2D, body_texture: Texture2D) -> Snake {
        Snake {
            head: (8, 8),
            head_texture: head_texture,
            body_texture: body_texture,
            dir: (1, 0),
            body: LinkedList::new(),
            dir_change_points: LinkedList::new(),
        }
    }
    pub fn reset(&mut self) {
        self.head = (8, 8);
        self.body = LinkedList::new();
        self.dir_change_points = LinkedList::new();
        self.dir = (1, 0);
    }
    fn draw_snake_part_two(&self, grid: &GameGrid, point: &Point, texture: &Texture2D) {
        let rotation = match self.dir.0 {
            //convert degrees to radians
            -1 => -90.0 * (PI / 180.0),
            0 => 0.0,
            //convert degrees to radians
            1 => 90.0 * (PI / 180.0),
            _ => 0.0,
        };
        draw_texture_ex(
            texture,
            grid.offset_x + point.0 as f32 * grid.square_size,
            grid.offset_y + point.1 as f32 * grid.square_size,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(grid.square_size, grid.square_size)),
                rotation: rotation,
                flip_y: self.dir.1 > 0,
                ..Default::default()
            },
        );
    }
    pub fn change_dir(&mut self, point: Point) {
        self.dir = point;
    }
    pub fn draw(&self, grid: &GameGrid) {
        //draw head
        Self::draw_snake_part_two(self, grid, &self.head, &self.head_texture);

        for point in &self.body {
            Self::draw_snake_part_two(self, grid, point, &self.body_texture);
            // Self::draw_snake_part(grid, point, GREEN);
        }
    }
}
