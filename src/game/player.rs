use barn::graphics::SdlRect;
use barn::graphics::color::Color;
use barn::graphics::SdlTexture;
use barn::graphics::barn_gfx::BarnGFX;
use barn::graphics::fill_type::FillType;
use barn::input::SdlKeycode;
use barn::math::bounding_box_2d::BoundingBox2D;
use barn::math::vector2::Vector2;
use barn::input::keyboard_handler::KeyboardHandler;

use crate::game::camera::Camera;
use crate::settings;

use std::collections::HashMap;

pub struct Player {
    pub width: u32,
    pub height: u32,
    pub pos: Vector2,
    pub vel: Vector2,
    pub xrect: i32,
    pub delay: f32,
    pub frame: i32,
    pub active_animation: String,
    pub animations: HashMap<String, Vec<SdlRect>>
}

impl Player {
    pub fn new() -> Player {
        Player {
            width: 36,
            height: 60,
            pos: Vector2::ZERO,
            vel: Vector2::ZERO,
            xrect: 0,
            delay: 13.0 / 60.0,
            frame: 0,
            active_animation: String::from("walk_down"),
            animations: Player::generate_animations(),
        }
    }

    pub fn get_bounding_box(&mut self) -> BoundingBox2D {
        BoundingBox2D { origin: self.pos, width: self.width, height: self.height }
    }

    pub fn generate_animations() -> HashMap<String, Vec<SdlRect>> {
        // Generate animation frames.
        let mut result: HashMap<String, Vec<SdlRect>> = HashMap::new();
        let mut up: Vec<SdlRect> = Vec::new();
        let mut down: Vec<SdlRect> = Vec::new();
        let mut left: Vec<SdlRect> = Vec::new();
        let mut right: Vec<SdlRect> = Vec::new();
        let mut up_idle: Vec<SdlRect> = Vec::new();
        let mut down_idle: Vec<SdlRect> = Vec::new();
        let mut left_idle: Vec<SdlRect> = Vec::new();
        let mut right_idle: Vec<SdlRect> = Vec::new();
        down.push(SdlRect::new(18, 0, 9, 15));
        down.push(SdlRect::new(9, 0, 9, 15));
        down.push(SdlRect::new(27, 0, 9, 15));
        down.push(SdlRect::new(9, 0, 9, 15));
        up.push(SdlRect::new(9, 15, 9, 15));
        up.push(SdlRect::new(0, 15, 9, 15));
        up.push(SdlRect::new(18, 15, 9, 15));
        up.push(SdlRect::new(0, 15, 9, 15));
        left.push(SdlRect::new(9, 30, 9, 15));
        left.push(SdlRect::new(0, 30, 9, 15));
        left.push(SdlRect::new(18, 30, 9, 15));
        left.push(SdlRect::new(0, 30, 9, 15));
        right.push(SdlRect::new(9, 30, 9, 15));
        right.push(SdlRect::new(0, 30, 9, 15));
        right.push(SdlRect::new(18, 30, 9, 15));
        right.push(SdlRect::new(0, 30, 9, 15));
        down_idle.push(SdlRect::new(0, 0, 9, 15));
        up_idle.push(SdlRect::new(0, 15, 9, 15));
        left_idle.push(SdlRect::new(27, 15, 9, 15));
        right_idle.push(SdlRect::new(27, 15, 9, 15));
        result.insert(String::from("walk_up"), up);
        result.insert(String::from("walk_down"), down);
        result.insert(String::from("walk_left"), left);
        result.insert(String::from("walk_right"), right);
        result.insert(String::from("idle_up"), up_idle);
        result.insert(String::from("idle_down"), down_idle);
        result.insert(String::from("idle_left"), left_idle);
        result.insert(String::from("idle_right"), right_idle);
        result
    }

    pub fn update(&mut self, input: &mut KeyboardHandler, dt: f32) {
        // Update movement
        let prev_anim = self.active_animation.clone();
        if input.key_pressed(&SdlKeycode::Left) && !input.key_pressed(&SdlKeycode::Right) {
            self.vel.x -= 225.0 * dt;
            if self.active_animation != "walk_left" {
                self.active_animation = String::from("walk_left");
            }
        } else if input.key_pressed(&SdlKeycode::Right) && !input.key_pressed(&SdlKeycode::Left) {
            self.vel.x += 225.0 * dt;
            if self.active_animation != "walk_right" {
                self.active_animation = String::from("walk_right");
            }
        }
        if input.key_pressed(&SdlKeycode::Up) && !input.key_pressed(&SdlKeycode::Down) {
            self.vel.y -= 225.0 * dt;
            if self.active_animation != "walk_up" {
                self.active_animation = String::from("walk_up");
            }
        } else if input.key_pressed(&SdlKeycode::Down) && !input.key_pressed(&SdlKeycode::Up) {
            self.vel.y += 225.0 * dt;
            if self.active_animation != "walk_down" {
                self.active_animation = String::from("walk_down");
            }
        }
        if !((input.key_pressed(&SdlKeycode::Left) && !input.key_pressed(&SdlKeycode::Right))
            || (input.key_pressed(&SdlKeycode::Right) && !input.key_pressed(&SdlKeycode::Left))
            || (input.key_pressed(&SdlKeycode::Up) && !input.key_pressed(&SdlKeycode::Down))
            || (input.key_pressed(&SdlKeycode::Down) && !input.key_pressed(&SdlKeycode::Up)))
        {
            if self.active_animation == "walk_left" {
                self.active_animation = String::from("idle_left")
            } else if self.active_animation == "walk_right" {
                self.active_animation = String::from("idle_right")
            } else if self.active_animation == "walk_up" {
                self.active_animation = String::from("idle_up")
            } else if self.active_animation == "walk_down" {
                self.active_animation = String::from("idle_down")
            }
        }
        if self.active_animation != prev_anim {
            self.frame = 0;
            self.delay = 13.0 / 60.0;
        }

        // Update animation
        self.delay -= dt;
        if self.delay <= 0.0 {
            self.frame += 1;
            if self.frame >= self.animations.get(&self.active_animation).unwrap().len() as i32 {
                self.frame = 0;
            }
            self.delay = 13.0 / 60.0
        }
    }

    pub fn draw_shadow(
        &mut self,
        texture: &mut SdlTexture,
        camera: &mut Camera,
        bgfx: &mut BarnGFX,
    ) {
        bgfx.sdl.draw_texture(texture, Some(SdlRect::new(0, 0, self.width, self.width)), Some(SdlRect::new(
            self.pos.x as i32 - camera.x,
            self.pos.y as i32 - camera.y + 24,
            self.width,
            self.height,
        )));
    }

    pub fn draw(&mut self, texture: &mut SdlTexture, camera: &mut Camera, bgfx: &mut BarnGFX) {
        bgfx.sdl.draw_texture_ex(
            texture, 
            Some(self.animations.get(&self.active_animation).unwrap()[self.frame as usize]), 
            Some(SdlRect::new(
                self.pos.x as i32 - camera.x,
                self.pos.y as i32- camera.y,
                self.width,
                self.height,
            )), 
            0.0, 
            Vector2::ZERO,
            if self.active_animation == "walk_left" || self.active_animation == "idle_left" {
                true
            } else {
                false
            },
            false,
        );

        // Render the collision box.
        if settings::DEBUG {
            bgfx.sdl.set_draw_color(Color::from_rgb(0, 220, 0));
            bgfx.sdl.draw_rect(self.pos.x as i32 - camera.x, self.pos.y as i32 - camera.y, self.width, self.height, FillType::LINE, false)
        }
    }
}
