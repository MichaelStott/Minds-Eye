use crate::camera::Camera;
use crate::input_handler::InputHandler;
use crate::settings;

use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::render::Texture;
use sdl2::keyboard::Keycode;

pub struct Player {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub velx: i32,
    pub vely: i32,
    pub xrect: i32,
    pub delay: i32,
    pub frame: i32,
    pub active_animation: String,
    pub animations: HashMap<String, Vec<Rect>>
}

impl Player {
    pub fn new() -> Player {
        Player {
            width: 36,
            height: 60,
            x: 26 + 128,
            y: 26 + 64 * 6,
            velx: 0,
            vely: 0,
            xrect: 0,
            delay: 15,
            frame: 0,
            active_animation: String::from("walk_down"),
            animations: Player::generate_animations()
        }
    }

    pub fn generate_animations() -> HashMap<String, Vec<Rect>> {
        // Generate animation frames.
        let mut result: HashMap<String, Vec<Rect>> = HashMap::new();
        let mut up: Vec<Rect> = Vec::new();
        let mut down: Vec<Rect> = Vec::new();
        let mut left: Vec<Rect> = Vec::new();
        let mut right: Vec<Rect> = Vec::new();
        let mut up_idle: Vec<Rect> = Vec::new();
        let mut down_idle: Vec<Rect> = Vec::new();
        let mut left_idle: Vec<Rect> = Vec::new();
        let mut right_idle: Vec<Rect> = Vec::new();
        down.push(Rect::new(18, 0, 9, 15));
        down.push(Rect::new(9, 0, 9, 15));
        down.push(Rect::new(27, 0, 9, 15));
        down.push(Rect::new(9, 0, 9, 15));
        up.push(Rect::new(9, 15, 9, 15));
        up.push(Rect::new(0, 15, 9, 15));
        up.push(Rect::new(18, 15, 9, 15));
        up.push(Rect::new(0, 15, 9, 15));
        left.push(Rect::new(9, 30, 9, 15));
        left.push(Rect::new(0, 30, 9, 15));
        left.push(Rect::new(18, 30, 9, 15));
        left.push(Rect::new(0, 30, 9, 15));
        right.push(Rect::new(9, 30, 9, 15));
        right.push(Rect::new(0, 30, 9, 15));
        right.push(Rect::new(18, 30, 9, 15));
        right.push(Rect::new(0, 30, 9, 15));
        down_idle.push(Rect::new(0, 0, 9, 15));
        up_idle.push(Rect::new(0, 15, 9, 15));
        left_idle.push(Rect::new(27, 15, 9, 15));
        right_idle.push(Rect::new(27, 15, 9, 15));
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

    pub fn update(&mut self, input: &mut InputHandler) {
        // Update movement
        let prev_anim = self.active_animation.clone();
        if input.key_pressed(&Keycode::Left) {
            self.velx -= 3;
            if self.active_animation != "walk_left" {
                self.active_animation = String::from("walk_left");
            }
        }
        if input.key_pressed(&Keycode::Right) {
            self.velx += 3;
            if self.active_animation != "walk_right" {
                self.active_animation = String::from("walk_right");
            }
        }
        if input.key_pressed(&Keycode::Up) {
            self.vely -= 3;
            if self.active_animation != "walk_up" {
                self.active_animation = String::from("walk_up");
            }
        }
        if input.key_pressed(&Keycode::Down) {
            self.vely += 3;
            if self.active_animation != "walk_down" {
                self.active_animation = String::from("walk_down");
            }
        }
        if !(input.key_pressed(&Keycode::Left) || input.key_pressed(&Keycode::Right) ||
                input.key_pressed(&Keycode::Up) || input.key_pressed(&Keycode::Down)) {
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
            self.delay = 15;
        }

        // Update animation
        self.delay -= 1;
        if self.delay == 0 {
            self.frame += 1;
            if self.frame >= self.animations.get(&self.active_animation).unwrap().len() as i32 {
                self.frame = 0;
            }
            self.delay = 15
        }
    }
    
    pub fn move_player(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&mut self, texture: &Texture, camera: &mut Camera, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 220, 0));
        canvas.copy_ex(
            &texture,
            self.animations.get(&self.active_animation).unwrap()[self.frame as usize],
            Rect::new(self.x - camera.x, self.y  - camera.y, self.width, self.height),
            0.0,
            None,
            if self.active_animation == "walk_left" || self.active_animation == "idle_left" { true } else { false },
            false
        ).unwrap();
        // Render the collision box.
        if settings::DEBUG {
            canvas.set_draw_color(Color::RGB(0, 220, 0));
            canvas.draw_rect(Rect::new(self.x - camera.x, self.y  - camera.y, self.width, self.height)).unwrap();
        }
    }
}
