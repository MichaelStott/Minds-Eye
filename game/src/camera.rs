pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            x: 0, y: 0, width: 0, height: 0
        }
    }

    pub fn focus(&mut self, foc_x: i32, foc_y: i32) {
        self.x = foc_x - (self.width >> 1);
        self.y = foc_y - (self.height >> 1);
    }
}