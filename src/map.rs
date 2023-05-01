use std::f32::consts::PI;
use macroquad::prelude::*;
use crate::player::Player;

pub struct Map {
    map_size_x: u8,
    map_size_y: u8,
    map_size: usize,
    map_array: [bool; 64]
}

struct Ray {
    x_start: f32,
    y_start: f32,
    x_end: f32,
    y_end: f32,
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

    pub fn display_rays(&self, player: &Player) {
        let pi_degrees: f32 = PI / 180.;

        let mut ray_angle: f32= player.get_angle() - pi_degrees * 30.;
        if ray_angle < 0. {
            ray_angle += 2. * PI;
        }

        if ray_angle > 2. * PI {
            ray_angle -= 2. * PI;
        }

        for _ray_index in 0..60 {
            let result_ray = self.compute_ray(player, ray_angle);

            if let Some(ray) = result_ray {
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

    fn compute_ray(&self, player : &Player, ray_angle : f32) -> Option<Ray> {
        let mut dof : u8 = 0;
        let ray_angle_cotangent: f32 = -1. / ray_angle.tan();
        let (mut ray_x, mut ray_y, mut ray_x_offset, mut ray_y_offset) : (f32, f32, f32, f32);

        let vertical_ray : Option<Ray>;
        let horizontal_ray : Option<Ray>;

        if ray_angle > PI { // Up
            ray_y = (((player.get_y_position() as i32) >> 6) << 6) as f32 - 0.0001;
            ray_x = (player.get_y_position() as f32 - ray_y) * ray_angle_cotangent + player.get_x_position() as f32;
            ray_y_offset = -64.;
        } else { // Down
            ray_y = (((player.get_y_position() as i32) >> 6) << 6) as f32 + 64.;
            ray_x = (player.get_y_position() as f32 - ray_y) * ray_angle_cotangent + player.get_x_position() as f32;
            ray_y_offset = 64.;
        }

        ray_x_offset = -ray_y_offset * ray_angle_cotangent;

        if ray_angle == 0. || ray_angle == PI { // Left or Right
            ray_x = player.get_x_position() as f32;
            ray_y = player.get_y_position() as f32;
        }

        while dof < self.map_size_x {
            let cell_map_x : u32 = ray_x as u32 >> 6;
            let cell_map_y : u32 = ray_y as u32 >> 6;
            let player_cell_map : u32 = cell_map_y * self.map_size_x as u32 + cell_map_x;

            if player_cell_map > 0 && player_cell_map < self.map_size as u32 && self.map_array[player_cell_map as usize] { // Hit
                dof = self.map_size_x;
            } else {
                ray_x += ray_x_offset;
                ray_y += ray_y_offset;
                dof += 1;
            }
        }

        if dof == self.map_size_x {
            horizontal_ray = Some(Ray {
                x_start: player.get_x_position() as f32,
                y_start: player.get_y_position() as f32,
                x_end: ray_x,
                y_end: ray_y,
            });
        } else {
            horizontal_ray = None;
        }

        dof = 0;
        let rays_angle_negative_tan: f32 = -ray_angle.tan();
        if ray_angle > (PI/2.) && ray_angle < (3. * PI / 2.)  { // Left
            ray_x = (((player.get_x_position() as i32) >> 6) << 6) as f32 - 0.0001;
            ray_y = (player.get_x_position() as f32 - ray_x) * rays_angle_negative_tan + player.get_y_position() as f32;
            ray_x_offset = -64.;
        } else if ray_angle < (PI/2.) || ray_angle > (3. * PI / 2.) { // Right
            ray_x = (((player.get_x_position() as i32) >> 6) << 6) as f32 + 64.;
            ray_y = (player.get_x_position() as f32 - ray_x) * rays_angle_negative_tan + player.get_y_position() as f32;
            ray_x_offset = 64.;
        }

        ray_y_offset = -ray_x_offset * rays_angle_negative_tan;

        if ray_angle == 0. || ray_angle == PI { // Up or Down
            ray_x = player.get_x_position() as f32;
            ray_y = player.get_y_position() as f32;
        }

        while dof < self.map_size_x {
            let ray_cell_x : u32 = ray_x as u32 >> 6;
            let ray_cell_y : u32  = ray_y as u32 >> 6;
            let player_cell_map : u32  = ray_cell_y * self.map_size_x as u32  + ray_cell_x;

            if player_cell_map > 0 && player_cell_map < self.map_size as u32 && self.map_array[player_cell_map as usize] { // Hit
                dof = self.map_size_x;
            } else {
                ray_x += ray_x_offset;
                ray_y += ray_y_offset;
                dof += 1;
            }
        }

        if dof == self.map_size_x {
            vertical_ray = Some(Ray {
                x_start: player.get_x_position() as f32,
                y_start: player.get_y_position() as f32,
                x_end: ray_x,
                y_end: ray_y,
            });
        } else {
            vertical_ray = None;
        }

        let mut result_ray : Option<Ray> = None;
        if horizontal_ray.is_some() && vertical_ray.is_some() {

            let mut hray : Ray = Ray {
                x_start: 0.0,
                y_start: 0.0,
                x_end: 0.0,
                y_end: 0.0,
            };
            let mut vray : Ray = Ray {
                x_start: 0.0,
                y_start: 0.0,
                x_end: 0.0,
                y_end: 0.0,
            };

            match horizontal_ray {
                Some(ray) => hray = ray,
                _ => {}
            }

            match vertical_ray {
                Some(ray) => vray = ray,
                _ => {}
            }

            let hray_distance : f32 = self.get_distance(&mut hray);
            let vray_distance : f32 = self.get_distance(&mut vray);

            if hray_distance < vray_distance {
                result_ray = Some(hray);
            } else {
                result_ray = Some(vray);
            }

        }
        return result_ray;
    }

    fn get_distance(&self, point: &mut Ray) -> f32 {
        let x_distance : f32 = point.x_start - point.x_end;
        let y_distance : f32 = point.y_start - point.y_end;
        (x_distance * x_distance + y_distance * y_distance).sqrt()
    }
}
