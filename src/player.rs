use std::f64::consts::PI;

pub struct Player {
    x: u32,
    y: u32,
    speed: u32,
    delta_x: f32,
    delta_y: f32,
    angle: f32,
    size: u32,
}

impl Player {
    pub fn new(x: u32, y: u32, speed : u32, size: u32) -> Player {
        Player { x, y, speed, size, delta_x: 0., delta_y: 0., angle: 0. }
    }

    pub fn get_x_position(&self) -> u32 {
        self.x
    }

    pub fn set_x_position(&mut self, x: u32) {
        self.x = x;
    }

    pub fn update_x_position(&mut self, x: i32) {
        if x > 0 {
            self.set_x_position(self.x.saturating_add(x as u32));
        } else {
            self.set_x_position(self.x.saturating_sub(x.abs() as u32));
        }
    }

    pub fn get_y_position(&self) -> u32 {
        self.y
    }

    pub fn set_y_position(&mut self, y: u32) {
        self.y = y;
    }

    pub fn update_y_position(&mut self, y: i32) {
        if y > 0 {
            self.set_y_position(self.y.saturating_add(y as u32));
        } else {
            self.set_y_position(self.y.saturating_sub(y.abs() as u32));
        }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_speed(&self) -> u32 {
        self.speed
    }

    pub fn get_delta_x(&self) -> f32 {
        self.delta_x
    }

    pub fn get_delta_y(&self) -> f32 {
        self.delta_y
    }

    pub fn move_forward(&mut self) {
        self.update_x_position(self.get_delta_x() as i32);
        self.update_y_position(self.get_delta_y() as i32);
        /*if (self.get_y_position() - self.get_speed()) >= (self.get_size() as f32/2.) as u32 {
            self.update_y_position(-(self.get_speed() as i32));
        } else {
            self.set_y_position((self.get_size() as f32/2.) as u32);
        }*/
    }

    pub fn move_backward(&mut self) {
        self.update_x_position(-(self.get_delta_x() as i32));
        self.update_y_position(-(self.get_delta_y() as i32));
        /*if (self.get_y_position() + (self.get_size() as f32/2.) as u32) < 512 {
            self.update_y_position(self.get_speed() as i32);
        } else {
            self.set_y_position(512 - (self.get_size() as f32/2.) as u32);
        }*/
    }

    pub fn turn_right(&mut self) {
        self.angle += 0.1;
        if self.angle > 2. * PI as f32 {
            self.angle = -2. * PI as f32;
        }
        self.delta_x = self.angle.cos() * self.get_speed() as f32;
        self.delta_y = self.angle.sin() * self.get_speed() as f32;
        /*if (self.get_x_position() + (self.get_size() as f32/2.) as u32) < 512 {
            self.update_x_position(self.get_speed() as i32);
        } else {
            self.set_x_position(512 - (self.get_size() as f32/2.) as u32);
        }*/
    }

    pub fn turn_left(&mut self) {
        self.angle -= 0.1;
        if self.angle < 0. {
            self.angle = 2. * PI as f32;
        }
        self.delta_x = self.angle.cos() * self.get_speed() as f32;
        self.delta_y = self.angle.sin() * self.get_speed() as f32;

        /*if (self.get_x_position() - self.get_speed()) >= (self.get_size() as f32/2.) as u32 {
            self.update_x_position(-(self.get_speed() as i32));
        } else {
            self.set_x_position((self.get_size() as f32/2.) as u32);
        }*/
    }
}