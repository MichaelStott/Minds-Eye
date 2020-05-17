pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub minx: i32,
    pub miny: i32,
    pub maxx: i32,
    pub maxy: i32
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            minx: -10000,
            miny: -10000,
            maxx: 10000,
            maxy: 10000
        }
    }

    pub fn focus(&mut self, foc_x: i32, foc_y: i32) {
        self.x = foc_x - (self.width >> 1);
        self.y = foc_y - (self.height >> 1);
        // if self.x < self.minx { self.x = self.minx }
        // else if self.x > self.maxx {self.x = self.maxx }
        // if self.y < self.miny { self.y = self.miny }
        // else if self.y > self.maxy { self.y = self.maxy }
    }
}
