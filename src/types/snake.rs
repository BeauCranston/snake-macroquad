use std::{collections::LinkedList, f32::consts::PI};

use macroquad::{
    color::{Color, DARKGREEN, GREEN, WHITE},
    math::{self, vec2},
    shapes::draw_rectangle,
    texture::{DrawTextureParams, Texture2D, draw_texture, draw_texture_ex, load_texture},
};

use crate::{Point, types::game_grid::GameGrid};

#[derive(Copy, Clone)]
pub struct VectorPoint {
    pub location: Point,
    pub dir: Point,
}

pub struct Snake {
    pub head: VectorPoint,
    pub head_texture: Texture2D,
    pub body_texture: Texture2D,
    pub body: LinkedList<VectorPoint>,
}

impl Snake {
    pub fn new(head_texture: Texture2D, body_texture: Texture2D) -> Snake {
        Snake {
            head: {
                VectorPoint {
                    location: (8, 8),
                    dir: (1, 0),
                }
            },
            head_texture: head_texture,
            body_texture: body_texture,
            body: LinkedList::new(),
        }
    }
    pub fn reset(&mut self) {
        self.head = VectorPoint {
            location: (8, 8),
            dir: (1, 0),
        };
        self.body = LinkedList::new();
    }
    fn draw_snake_part(grid: &GameGrid, point: &VectorPoint, texture: &Texture2D) {
        let rotation = match point.dir.0 {
            //convert degrees to radians
            -1 => -90.0 * (PI / 180.0),
            0 => 0.0,
            //convert degrees to radians
            1 => 90.0 * (PI / 180.0),
            _ => 0.0,
        };
        draw_texture_ex(
            texture,
            grid.offset_x + point.location.0 as f32 * grid.square_size,
            grid.offset_y + point.location.1 as f32 * grid.square_size,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(grid.square_size, grid.square_size)),
                rotation: rotation,
                flip_y: point.dir.1 > 0,
                ..Default::default()
            },
        );
    }
    pub fn change_dir(&mut self, point: Point) {
        self.head.dir = point;
    }
    //change body direction:
    //push change point when direction changes ^
    //if the point is on a direction change, change the direction to the points dir
    //if the tail of the snake has reached the direction change point, pop it off the stack
    //how do I know if the body point has hit a direction change? (loop?, refactor!)
    pub fn draw(&self, grid: &GameGrid) {
        //draw head
        Self::draw_snake_part(grid, &self.head, &self.head_texture);

        for point in &self.body {
            Self::draw_snake_part(grid, point, &self.body_texture);
            // Self::draw_snake_part(grid, point, GREEN);
        }
    }
}
