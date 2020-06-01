use crate::camera::Camera;
use std::collections::HashMap;

use rand::distributions::{Distribution, Uniform};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;

pub struct Fire {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub delay: i32,
    pub frame: i32,
    pub active_animation: String,
    pub animations: HashMap<String, Vec<Rect>>,
}

impl Fire {
    pub fn new() -> Fire {
        Fire {
            width: 24,
            height: 42, 
            x: 26 + 128,
            y: 26 + 64 * 6,
            delay: 10,
            frame: Uniform::from(1..2).sample(&mut rand::thread_rng()),
            active_animation: String::from("flame"),
            animations: Fire::generate_animations(),
        }
    }

    pub fn generate_animations() -> HashMap<String, Vec<Rect>> {
        let mut result: HashMap<String, Vec<Rect>> = HashMap::new();
        let mut flame: Vec<Rect> = Vec::new();
        let mut glow: Vec<Rect> = Vec::new();
        flame.push(Rect::new(0,0,8,14));
        flame.push(Rect::new(8,0,8,14));
        result.insert(String::from("flame"), flame);
        glow.push(Rect::new(0,0,32,32));
        glow.push(Rect::new(32,0,32,32));
        result.insert(String::from("glow"), glow);
        result
    }


    pub fn update(&mut self) {
        // Update animation
        self.delay -= 1;
        if self.delay == 0 {
            self.frame += 1;
            if self.frame >= self.animations.get(&self.active_animation).unwrap().len() as i32 {
                self.frame = 0;
            }
            self.delay = 5
        }
    }


    pub fn draw(
        &mut self,
        texture: &Texture,
        glow_texture: &Texture,
        camera: &mut Camera,
        canvas: &mut WindowCanvas,
    ) {
        canvas
            .copy_ex(
                &texture,
                self.animations.get(&self.active_animation).unwrap()[self.frame as usize],
                Rect::new(
                    self.x - camera.x,
                    self.y - camera.y,
                    self.width,
                    self.height,
                ),
                0.0,
                None,
                false,
                false,
            )
            .unwrap();
        canvas
            .copy(
                &glow_texture,
                self.animations.get("glow").unwrap()[self.frame as usize],
                Some(Rect::new(
                    self.x  - camera.x - 32 + self.width as i32 / 2,
                    self.y - camera.y - 4,
                    64,
                    64,
                )),
            )
            .unwrap();
    }
}