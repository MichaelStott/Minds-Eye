extern crate rand;

use barn::graphics::barn_gfx::BarnGFX;
use crate::game::camera::Camera;
use crate::game::tile::Tile;

use sdl2::rect::Rect;
use sdl2::render::Texture;

use rand::Rng;

pub struct Eye {
    pub direction: String,
    pub color: String,
    pub solved: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub deltax: f32,
    pub deltay: f32,
    pub anger: u8,
}

impl Eye {
    pub fn update(&mut self, tiles: &mut Vec<Tile>, dt: f32) {
        let mut rng = rand::thread_rng();
        self.solved = false;
        let mut distance = -1.0;
        let mut isblock = false;
        // Check for the closest block.
        for tile in tiles.iter_mut() {
            if tile.bb.origin.y == self.y as f32 && self.x as f32 > tile.bb.origin.x && (tile.isblock || tile.iswall) {
                if distance == -1.0 || self.x as f32 - tile.bb.origin.x < distance {
                    distance = self.x as f32 - tile.bb.origin.x;
                    isblock = tile.isblock && tile.texture.contains(&self.color);
                    self.direction = String::from("left");
                }
            }
        }
        if !isblock {
            distance = -1.0;
            for tile in tiles.iter_mut() {
                if tile.bb.origin.y == self.y as f32 && (self.x as f32) < tile.bb.origin.x && (tile.isblock || tile.iswall) {
                    if distance == -1.0 || tile.bb.origin.x - (self.x as f32) < distance {
                        distance = tile.bb.origin.x - self.x as f32;
                        isblock = tile.isblock && tile.texture.contains(&self.color);
                        self.direction = String::from("right");
                    }
                }
            }
        }
        if !isblock {
            distance = -1.0;
            for tile in tiles.iter_mut() {
                if tile.bb.origin.x == self.x as f32 && (self.y as f32) < tile.bb.origin.y && (tile.isblock || tile.iswall) {
                    if distance == -1.0 || tile.bb.origin.y - (self.y as f32) < distance {
                        distance = tile.bb.origin.y - self.y as f32;
                        isblock = tile.isblock && tile.texture.contains(&self.color);
                        self.direction = String::from("up");
                    }
                }
            }
        }
        if !isblock {
            distance = -1.0;
            for tile in tiles.iter_mut() {
                if tile.bb.origin.x == self.x as f32 && self.y as f32 > tile.bb.origin.y && (tile.isblock || tile.iswall) {
                    if distance == -1.0 || self.y as f32 - tile.bb.origin.y < distance {
                        distance = self.y as f32- tile.bb.origin.y;
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
                self.deltax = if self.deltax > -12.0 {
                    self.deltax - dt * 100.0
                } else {
                    -12.0
                };
                self.deltay = 0.0;
                self.anger = if self.anger != 255 && self.anger as i32 + 10 < 255 {
                    self.anger + 10
                } else {
                    255
                };
            } else if self.direction == "right" {
                self.solved = true;
                self.deltax = if self.deltax < 12.0 {
                    self.deltax + dt * 100.0
                } else {
                    12.0
                };
                self.deltay = 0.0;
                self.anger = if self.anger != 255 && self.anger as i32 + 10 < 255 {
                    self.anger + 10
                } else {
                    255
                };
            } else if self.direction == "up" {
                self.solved = true;
                self.deltay = if self.deltay < 6.0 { self.deltay + dt * 100.0 } else { 6.0 };
                self.deltax = 0.0;
                self.anger = if self.anger != 255 && self.anger as i32 + 10 < 255 {
                    self.anger + 10
                } else {
                    255
                };
            } else if self.direction == "down" {
                self.solved = true;
                self.deltay = if self.deltay > -6.0 {
                    self.deltay - dt * 100.0
                } else {
                    -6.0
                };
                self.deltax = 0.0;
                self.anger = if self.anger != 255 && self.anger as i32 + 10 < 255 {
                    self.anger + 10
                } else {
                    255
                };
            }
        }
        // Jitter the pupil if unsolved.
        if !self.solved {
            self.deltax = rng.gen_range(-3.0, 3.0);
            self.deltay = rng.gen_range(-3.0, 3.0);
            self.anger = if self.anger != 0 && self.anger as i32 - 10 > 0 {
                self.anger - 10
            } else {
                0
            };
        }
    }

    pub fn draw_socket(
        &mut self,
        tex_socket: &mut Texture,
        camera: &mut Camera,
        bgfx: &mut BarnGFX,
    ) {
        if self.color == "red" {
            tex_socket.set_color_mod(255, self.anger, self.anger);
        } else if self.color == "green" {
            tex_socket.set_color_mod(self.anger, 255, self.anger);
        } else if self.color == "blue" {
            tex_socket.set_color_mod(self.anger, self.anger, 255);
        }

        bgfx.sdl.draw_texture(tex_socket, 
            None, 
            Some(Rect::new(
                self.x - camera.x,
                self.y - camera.y,
                self.width,
                self.height,
            )));
    }

    pub fn draw_iris(
        &mut self,
        tex_pupil: &mut Texture,
        camera: &mut Camera,
        bgfx: &mut BarnGFX,
    ) {
        bgfx.sdl.draw_texture(tex_pupil, 
            None, 
            Some(Rect::new(
                self.x - camera.x + self.deltax as i32,
                self.y - camera.y + self.deltay as i32,
                self.width,
                self.height,
            )));
    }
}
