use macroquad::prelude::*;
use std::collections::LinkedList;

pub struct GameGrid {
    pub grid_size: f32,
    pub screen_height: f32,
    pub screen_width: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub square_size: f32,
}

impl GameGrid {
    pub fn new(screen_width: f32, screen_height: f32, grid_size: u32) -> GameGrid {
        //using this to create a square by taking the min between height and width
        let game_size = screen_width.min(screen_height);
        //offset between screen end and grid end
        let offset_x = (screen_width - game_size) / 2. + 10.;
        //offset between screen end and grid end
        let offset_y = (screen_height - game_size) / 2. + 10.;
        //individual size of square in grid
        let square_size = (screen_height - offset_y * 2.) / grid_size as f32;

        GameGrid {
            screen_width,
            screen_height,
            offset_x,
            offset_y,
            square_size,
            grid_size: grid_size as f32,
        }
    }

    pub fn draw(&self) {
        for i in 0..(self.grid_size + 1.) as i32 {
            //draw the horizontal lines
            draw_line(
                0. + self.offset_x,
                self.offset_y + self.square_size * i as f32,
                self.screen_width - self.offset_x,
                self.offset_y + self.square_size * i as f32,
                1.,
                BLACK,
            );

            //draw the vertical lines
            draw_line(
                self.offset_x + self.square_size * i as f32,
                0. + self.offset_y,
                self.offset_x + self.square_size * i as f32,
                self.screen_height - self.offset_y,
                1.,
                BLACK,
            );
        }
    }
}
