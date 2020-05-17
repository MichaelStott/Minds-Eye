use crate::camera::Camera;
use crate::settings;

use std::ptr;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;

#[derive(Clone)]
pub struct Tile {
    pub texture: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub targetx: i32,
    pub targety: i32,
    pub resistancex: u32,
    pub resistancey: u32,
    pub iswall: bool,
    pub isblock: bool,
}

impl Tile {
    pub fn update(&mut self, tiles: &Vec<Tile>) {
        let prevx = self.x;
        let prevy = self.y;
        if self.targetx != self.x {
            let dir = (self.targetx - self.x) / (self.targetx - self.x).abs();
            let delta = 3;
            self.x = if dir * (self.x + delta * dir) > dir * self.targetx {
                self.targetx
            } else {
                self.x + delta * dir
            };
        }
        for tile in tiles {
            if (tile.iswall || tile.isblock) && !(prevx == tile.x && prevy == tile.y) && does_intersect(tile, self) {
                self.x = prevx;
                self.resistancex = 30;
                self.targetx = self.x;
            }
        }
        if self.targety != self.y {
            let dir = (self.targety - self.y) / (self.targety - self.y).abs();
            let delta = 3;

            self.y = if dir * (self.y + delta * dir) > dir * self.targety {
                self.targety
            } else {
                self.y + delta * dir
            };
        }
        for tile in tiles {
            if (tile.iswall || tile.isblock) && !(prevx == tile.x && prevy == tile.y) && does_intersect(tile, self) {
                self.y = prevy;
                self.resistancey = 30;
                self.targety = self.y;
            }
        }
    }

    pub fn draw(&mut self, texture: &Texture, camera: &mut Camera, canvas: &mut WindowCanvas) {
        canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(
                    self.x - camera.x,
                    self.y - camera.y,
                    self.width,
                    self.height,
                )),
            )
            .unwrap();
        // Render the collision box.
        if settings::DEBUG {
            canvas.set_draw_color(Color::RGB(220, 220, 220));
            canvas
                .draw_rect(Rect::new(
                    self.x - camera.x,
                    self.y - camera.y,
                    self.width,
                    self.height,
                ))
                .unwrap();
        }
    }
}

pub fn does_intersect(player: &Tile, tile: &Tile) -> bool {
    (player.x < tile.x + tile.width as i32)
        && (player.x + player.width as i32 > tile.x)
        && (player.y < tile.y + tile.height as i32)
        && (player.y + player.height as i32 > tile.y)
}
