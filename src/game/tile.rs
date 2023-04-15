use barn::graphics::color::Color;
use barn::graphics::SdlTexture;
use barn::graphics::barn_gfx::BarnGFX;
use barn::graphics::fill_type::FillType;
use barn::math::vector2::Vector2;
use barn::math::bounding_box_2d::BoundingBox2D;
use crate::game::camera::Camera;
use crate::settings;
use sdl2::mixer::Chunk;

use sdl2::rect::Rect;

#[derive(Clone)]
pub struct Tile {
    pub texture: String,
    pub bb: BoundingBox2D,
    pub target_pos: Vector2,
    pub resistance: f32,
    pub iswall: bool,
    pub isblock: bool,
    pub moving: bool
}

impl Tile {

    pub fn update(&mut self, tiles: &Vec<Tile>, move_fx: &Chunk, dt: f32) {
        let prev_pos = self.bb.origin.clone();
        let delta = 200.0;
        let prev_moving = self.moving;
        // If a new position is assigned, tween to that position.
        if self.target_pos != self.bb.origin {
            let mut diff = self.target_pos.clone() - self.bb.origin.clone();
            let dist = diff.length();
            if delta * dt < dist {
                self.moving = true;
                self.bb.origin += diff.normalize() *  delta * dt;
            } else {
                self.moving = false;
                self.bb.origin = self.target_pos;
            }
        }
        // Prevent tile from moving into wall or another block
        for tile in tiles {
            if (tile.iswall || tile.isblock)
            && tile.bb.origin != self.bb.origin
            && tile.bb.origin != prev_pos
            && self.bb.intersects_box(&mut tile.bb.clone()) {
                //self.bb.origin = prev_pos.clone();
                self.resistance = 30.0;
                self.bb.origin = prev_pos;
                self.target_pos = prev_pos;
                self.moving = false;
            }
        }

        if settings::ENABLE_SOUND && !prev_moving && self.moving {
            let channel = sdl2::mixer::Channel(0);
            channel.play(&move_fx, 0);     
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
        if settings::DEBUG && (self.iswall || self.isblock) {
            let mut color = Color::from_rgb(220, 220, 220);
            if self.isblock {
                color = Color::from_rgb(220, 0, 0);
            }
            bgfx.sdl.set_draw_color(color);
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

