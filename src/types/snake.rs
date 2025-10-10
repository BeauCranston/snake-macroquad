use std::{collections::LinkedList, f32::consts::PI};

use macroquad::{
    color::{Color, DARKGREEN, GREEN, WHITE},
    math::{self, vec2},
    shapes::draw_rectangle,
    texture::{DrawTextureParams, Texture2D, draw_texture, draw_texture_ex, load_texture},
};

use crate::{Point, types::game_grid::GameGrid};

//struct that stores the location and direction
#[derive(Copy, Clone)]
pub struct VectorPoint {
    pub location: Point,
    pub dir: Point,
}

// snake struct
pub struct Snake {
    pub head: VectorPoint,
    pub head_texture: Texture2D,
    pub body_texture: Texture2D,
    //body is list of vector points so that the body sections know when to change direction
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
    //when the game resets, modify snake instance to starting values
    pub fn reset(&mut self) {
        self.head = VectorPoint {
            location: (8, 8),
            dir: (1, 0),
        };
        self.body = LinkedList::new();
    }
    //draws a part of the snake based on it's vector point, and texture (head or body)
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
    //change the direction of the head of the snake
    pub fn change_dir(&mut self, point: Point) {
        self.head.dir = point;
    }

    pub fn check_self_eat(&self) -> bool {
        let mut eating_self = false;
        for node in &self.body {
            if self.head.location.0 == node.location.0 && self.head.location.1 == node.location.1 {
                eating_self = true;
            }
        }
        eating_self
    }

    pub fn move_snake(&mut self, eating_fruit: bool) {
        let old_head = self.head.clone();
        //move snake head to new position
        self.head.location = (
            self.head.location.0 + self.head.dir.0,
            self.head.location.1 + self.head.dir.1,
        );

        //push old head to body
        self.body.push_front(old_head);
        //pop back if not eating fruit
        if !eating_fruit {
            self.body.pop_back();
        }
    }
    //draw the snake to the screen
    pub fn draw(&self, grid: &GameGrid) {
        //draw head
        Self::draw_snake_part(grid, &self.head, &self.head_texture);

        for point in &self.body {
            Self::draw_snake_part(grid, point, &self.body_texture);
            // Self::draw_snake_part(grid, point, GREEN);
        }
    }
}
