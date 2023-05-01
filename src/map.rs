use macroquad::prelude::*;
use crate::player::Player;

pub struct Map {
    map_size_x: u8,
    map_size_y: u8,
    map_size: usize,
    map_array: [bool; 64],
}

impl Map {
    pub fn new(size_x : u8, size_y : u8, map_array : [bool; 64]) -> Map {
        Map {
            map_size_x: size_x,
            map_size_y: size_y,
            map_size: (size_x * size_y) as usize,
            map_array,
        }
    }

    pub fn move_player(&self, player : Player) {
        draw_rectangle(
            player.get_x_position() as f32 - (player.get_size() as f32 / 2.),
            player.get_y_position() as f32 - (player.get_size() as f32 / 2.),
            player.get_size() as f32,
            player.get_size() as f32,
            DARKGREEN,
        );
    }

    pub fn draw(&self) {
        for pos_x in 0..self.map_size_x {
            for pos_y in 0..self.map_size_y {
                if self.map_array[(pos_x + pos_y * self.map_size_x) as usize] {
                    draw_rectangle(
                        pos_x as f32 * self.map_size as f32,
                        pos_y as f32 * self.map_size as f32,
                        self.map_size as f32,
                        self.map_size as f32,
                        DARKGRAY,
                    );
                }
            }
        }
    }
}
