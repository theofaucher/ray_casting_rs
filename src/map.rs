use std::f32::consts::PI;
use macroquad::prelude::*;
use crate::player::Player;
use crate::ray::{compute_ray, get_distance};

pub struct Map {
    map_size_x: u8,
    map_size_y: u8,
    map_size: usize,
    map_array: [bool; 64]
}

impl Map {
    pub fn new(size_x: u8, size_y: u8, map_array: [bool; 64]) -> Map {
        Map {
            map_size_x: size_x,
            map_size_y: size_y,
            map_size: (size_x * size_y) as usize,
            map_array,
        }
    }

    pub fn move_player(&self, player: &Player) {
        draw_rectangle(
            player.get_x_position() as f32 - (player.get_size() as f32 / 2.),
            player.get_y_position() as f32 - (player.get_size() as f32 / 2.),
            player.get_size() as f32,
            player.get_size() as f32,
            DARKGREEN,
        );

        draw_line(
            player.get_x_position() as f32,
            player.get_y_position() as f32,
            player.get_x_position() as f32 + player.get_delta_x() * 5.,
            player.get_y_position() as f32 + player.get_delta_y() * 5.,
            5.,
            DARKGREEN,
        );
    }

    pub fn draw_minimap(&self) {
        for pos_x in 0..self.map_size_x {
            for pos_y in 0..self.map_size_y {
                if self.map_array[(pos_x + pos_y * self.map_size_x) as usize] {
                    draw_rectangle(
                        pos_x as f32 * self.map_size as f32,
                        pos_y as f32 * self.map_size as f32,
                        self.map_size as f32,
                        self.map_size as f32,
                        BLACK,
                    );
                } else {
                    draw_rectangle(
                        pos_x as f32 * self.map_size as f32,
                        pos_y as f32 * self.map_size as f32,
                        self.map_size as f32,
                        self.map_size as f32,
                        WHITE,
                    );
                }
            }
        }

    }

    pub fn draw_3d(&self, player: &Player) {
        let pi_degrees: f32 = PI / 180.;

        let mut ray_angle: f32= player.get_angle() - pi_degrees * 30.;
        if ray_angle < 0. {
            ray_angle += 2. * PI;
        }

        if ray_angle > 2. * PI {
            ray_angle -= 2. * PI;
        }

        for ray_index in 0..60 {
            let result_ray = compute_ray(self, player, ray_angle);

            if let Some(mut ray) = result_ray {
                let mut ray_distance: f32 = get_distance(&mut ray);

                let mut angle_fix_fisheye: f32 = player.get_angle() - ray_angle;
                if angle_fix_fisheye < 0. {
                    angle_fix_fisheye += 2. * PI;
                }

                if angle_fix_fisheye > 2. * PI {
                    angle_fix_fisheye -= 2. * PI;
                }

                ray_distance = ray_distance * angle_fix_fisheye.cos();

                let mut line_height : f32 = self.map_size as f32 * 320. / ray_distance;
                if line_height > 320. {
                    line_height = 320.;
                }

                let line_x_offset : f32= 160. - line_height / 2.;

                draw_line(ray_index as f32 * 8. + 530.,
                          line_x_offset,
                          ray_index as f32 * 8. + 530.,
                          line_height + line_x_offset,
                          8.,
                          RED);

                draw_line(
                    ray.x_start,
                    ray.y_start,
                    ray.x_end,
                    ray.y_end,
                    2.,
                    DARKGREEN,
                );
            }

            ray_angle += pi_degrees;
        }
    }

    pub fn get_map_size_x(&self) -> u8 {
        self.map_size_x
    }
    pub fn get_map_size(&self) -> usize {
        self.map_size
    }
    pub fn get_map_array(&self) -> [bool; 64] {
        self.map_array
    }
}
