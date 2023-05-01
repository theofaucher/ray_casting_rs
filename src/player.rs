#[derive(Clone, Copy)]
pub struct Player {
    x: u32,
    y: u32,
    size: u32,
}

impl Player {
    pub fn new(x: u32, y: u32, size: u32) -> Player {
        Player { x, y, size }
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
}