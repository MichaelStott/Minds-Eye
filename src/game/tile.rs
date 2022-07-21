use barn::graphics::color::Color;
use barn::graphics::SdlTexture;
use barn::graphics::barn_gfx::BarnGFX;
use barn::graphics::fill_type::FillType;
use barn::math::vector2::Vector2;
use barn::math::bounding_box_2d::BoundingBox2D;
use crate::game::camera::Camera;
use crate::settings;
use sdl2::mixer::Chunk;

use std::ptr;

use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;

#[derive(Clone)]
pub struct Tile {
    pub texture: String,
    pub bb: BoundingBox2D,
    pub target_pos: Vector2,
    pub resistance: i32,
    pub iswall: bool,
    pub isblock: bool,
}

impl Tile {

    pub fn update(&mut self, tiles: &Vec<Tile>, move_fx: &Chunk) {
        let prev_pos = self.bb.origin.clone();
        let delta = 3.0;
        // If a new position is assigned, tween to that position.
        if self.target_pos != self.bb.origin {
            let mut diff = self.target_pos.clone() - self.bb.origin.clone();
            let dist = diff.length();
            let dir = diff.normalize();
            self.bb.origin += dir *  delta
        }
        // Check if tile needs to be moved.
        let mut intersected = false;
        for tile in tiles {
            // if (tile.iswall || tile.isblock)
            //     && prev_pos != self.bb.origin
            //     && self.bb.intersects_box(tile.bb.clone()) {
            //     self.bb.origin = prev_pos.clone();
            //     self.resistance = 30;
            //     self.target_pos = self.bb.origin.clone();
            //     intersected = true;
            //     let channel = sdl2::mixer::Channel(0);
            //     channel.play(&move_fx, 0);
            // }
        }
    }

    pub fn draw(&mut self, texture: &mut SdlTexture, camera: &mut Camera, bgfx: &mut BarnGFX) {
        bgfx.sdl.draw_texture(texture,  None, Some(Rect::new(
            self.bb.origin.x.round() as i32 - camera.x,
            self.bb.origin.y.round() as i32 - camera.y,
            self.bb.width as u32,
            self.bb.height as u32,
        )));
        
        // Render the collision box.
        if settings::DEBUG {
            bgfx.sdl.set_draw_color(Color::from_rgb(220, 220, 220));
            bgfx.sdl.draw_rect(self.bb.origin.x.round() as i32 - camera.x,
                self.bb.origin.y.round() as i32 - camera.y,  
                self.bb.width as u32,
                self.bb.height as u32, 
                FillType::LINE, 
                false);
        }
    }

    pub fn has_moved(&mut self) -> bool {
        (self.target_pos.y - self.bb.origin.y).abs() == self.bb.height as f32
            || (self.target_pos.x - self.bb.origin.x).abs() == self.bb.width as f32
    }
}

