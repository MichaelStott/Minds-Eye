extern crate rand;

use crate::camera::Camera;
use crate::tile::Tile;

use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;

use rand::Rng;

pub struct Eye {
    pub direction: String, // Unused variable.
    pub color: String,
    pub solved: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub deltax: i32,
    pub deltay: i32,
    pub anger: u8,
}

impl Eye {
    pub fn update(&mut self, tiles: &mut Vec<Tile>) {
        let mut rng = rand::thread_rng();
        self.solved = false;
        let mut distance = -1;
        let mut isblock = false;
        // Check for the closest block.
        for tile in tiles.iter_mut() {
            if tile.y == self.y && self.x > tile.x && (tile.isblock || tile.iswall) {
                if distance == -1 || self.x - tile.x < distance {
                    distance = self.x - tile.x;
                    isblock = tile.isblock && tile.texture.contains(&self.color);
                    self.direction = String::from("left");
                }
            }
        }
        if !isblock {
            distance = -1;
            for tile in tiles.iter_mut() {
                if tile.y == self.y && self.x < tile.x && (tile.isblock || tile.iswall) {
                    if distance == -1 || tile.x - self.x < distance {
                        distance = tile.x - self.x;
                        isblock = tile.isblock && tile.texture.contains(&self.color);
                        self.direction = String::from("right");
                    }
                }
            }
        }
        if !isblock {
            distance = -1;
            for tile in tiles.iter_mut() {
                if tile.x == self.x && self.y < tile.y && (tile.isblock || tile.iswall) {
                    if distance == -1 || tile.y - self.y < distance {
                        distance = tile.y - self.y;
                        isblock = tile.isblock && tile.texture.contains(&self.color);
                        self.direction = String::from("up");
                    }
                }
            }
        }
        if !isblock {
            distance = -1;
            for tile in tiles.iter_mut() {
                if tile.x == self.x && self.y > tile.y && (tile.isblock || tile.iswall) {
                    if distance == -1 || self.y - tile.y < distance {
                        distance = self.y - tile.y;
                        isblock = tile.isblock && tile.texture.contains(&self.color);
                        self.direction = String::from("down");
                    }
                }
            }
        }

        // If we found a valid block, mark as solved.
        if isblock {
            if self.direction == "left" {
                self.solved = true;
                self.deltax = if self.deltax > -12 {
                    self.deltax - 1
                } else {
                    -12
                };
                self.deltay = 0;
                self.anger = if self.anger != 255 && self.anger as i32 + 10 < 255 {
                    self.anger + 10
                } else {
                    255
                };
            } else if self.direction == "right" {
                self.solved = true;
                self.deltax = if self.deltax < 12 {
                    self.deltax + 1
                } else {
                    12
                };
                self.deltay = 0;
                self.anger = if self.anger != 255 && self.anger as i32 + 10 < 255 {
                    self.anger + 10
                } else {
                    255
                };
            } else if self.direction == "up" {
                self.solved = true;
                self.deltay = if self.deltay < 6 { self.deltay + 1 } else { 6 };
                self.deltax = 0;
                self.anger = if self.anger != 255 && self.anger as i32 + 10 < 255 {
                    self.anger + 10
                } else {
                    255
                };
            } else if self.direction == "down" {
                self.solved = true;
                self.deltay = if self.deltay > -6 {
                    self.deltay - 1
                } else {
                    -6
                };
                self.deltax = 0;
                self.anger = if self.anger != 255 && self.anger as i32 + 10 < 255 {
                    self.anger + 10
                } else {
                    255
                };
            }
        }
        // Jitter the pupil if unsolved.
        if !self.solved {
            self.deltax = rng.gen_range(-3, 3);
            self.deltay = rng.gen_range(-3, 3);
            self.anger = if self.anger != 0 && self.anger as i32 - 10 > 0 {
                self.anger - 10
            } else {
                0
            };
        }
    }

    pub fn draw(
        &mut self,
        tex_socket: &mut Texture,
        tex_pupil: &Texture,
        camera: &mut Camera,
        canvas: &mut WindowCanvas,
    ) {
        if self.color == "red" {
            tex_socket.set_color_mod(255, self.anger, self.anger);
        } else if self.color == "green" {
            tex_socket.set_color_mod(self.anger, 255, self.anger);
        } else if self.color == "blue" {
            tex_socket.set_color_mod(self.anger, self.anger, 255);
        }
        
        canvas
            .copy(
                &tex_socket,
                None,
                Some(Rect::new(
                    self.x - camera.x,
                    self.y - camera.y,
                    self.width,
                    self.height,
                )),
            )
            .unwrap();
        canvas
            .copy(
                &tex_pupil,
                None,
                Some(Rect::new(
                    self.x - camera.x + self.deltax,
                    self.y - camera.y + self.deltay,
                    self.width,
                    self.height,
                )),
            )
            .unwrap();
    }
}
