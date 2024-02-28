use barn::audio::load_music;
use barn::graphics::barn_gfx::BarnGFX;
use barn::game::barn_context::BarnContext;
use barn::graphics::color::Color;
use barn::graphics::fill_type::FillType;
use barn::math::vector2::Vector2;
use barn::math::bounding_box_2d::BoundingBox2D;
use barn::game::state::State;
use crate::game::camera::Camera;
use crate::game::credits_state::CreditsState;
use crate::game::eye::Eye;
use crate::game::help_state::HelpState;
use crate::game::level_select_state::LevelSelectState;
use crate::game::tile::Tile;
use crate::settings;

use sdl2::keyboard::Keycode;

use std::collections::HashMap;

pub struct StartMenuState {
    pub selected_option: u32,
    pub tiles: Vec<Tile>,
    pub eyes: Vec<Eye>,
    pub camera: Camera,
}

impl StartMenuState {
    pub fn new(selected_option: u32) -> Self {
        StartMenuState {
            selected_option: selected_option, 
            tiles: Vec::new(),
            eyes: Vec::new(),
            camera: Camera::new()
        }
    }
}

impl State<BarnContext> for StartMenuState {
    fn update(&mut self, context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> {
        for eye in self.eyes.iter_mut() {
            eye.update(&mut self.tiles, dt);
        }
        let prev_option = self.selected_option;
        if context.input.key_just_pressed(&Keycode::Down) {
            if self.selected_option == 2 {
                self.selected_option = 0;
            } else {
                self.selected_option += 1;
            }
        } else if context.input.key_just_pressed(&Keycode::Up) {
            if self.selected_option == 0 {
                self.selected_option = 2;
            } else {
                self.selected_option -= 1;
            }
        }
       
        if prev_option != self.selected_option {
            if settings::ENABLE_SOUND {
                let channel = sdl2::mixer::Channel(1);
                let select_fx = context.load_sound(String::from("res/sound/select.ogg"));
                channel.play(select_fx, 0).unwrap();
            }
            
            self.tiles.clear();
            if self.selected_option == 0 {
                self.tiles.push(Tile {
                    texture: String::from("res/img/blueblock.png"),
                    bb: BoundingBox2D {origin: Vector2 {x: 200.0, y: 200.0}, width: 64, height: 64},
                    target_pos: Vector2 {x: 200.0, y: 200.0},
                    resistance: 30.0,
                    iswall: false,
                    isblock: true,
                    moving: false, 
                });
            } else if self.selected_option == 1 {
                self.tiles.push(Tile {
                    texture: String::from("res/img/greenblock.png"),
                    bb: BoundingBox2D {origin: Vector2 {x: 200.0, y: 300.0}, width: 64, height: 64},
                    target_pos: Vector2 {x: 200.0, y: 500.0},
                    resistance: 30.0,
                    iswall: false,
                    isblock: true,
                    moving: false, 
                });
            } else if self.selected_option == 2 {
                self.tiles.push(Tile {
                    texture: String::from("res/img/redblock.png"),
                    bb: BoundingBox2D {origin: Vector2 {x: (self.camera.width / 2 - 32) as f32, y: 300.0}, width: 64, height: 64},
                    target_pos: Vector2 {x: 200.0, y: 200.0},
                    resistance: 30.0,
                    iswall: false,
                    isblock: true,
                    moving: false, 
                });
            }
        }
       
        if context.input.key_just_pressed(&Keycode::Return) {
            if settings::ENABLE_SOUND {
                let enter_fx = context.load_sound(String::from("res/sound/enter.ogg"));
                let channel = sdl2::mixer::Channel(2);
                channel.play(enter_fx, 0).unwrap();
            }
            
            if self.selected_option == 0 {
                return Some(Box::new(LevelSelectState {
                    levels: HashMap::new(),
                    options: Vec::new(),
                    selected_option: 0,
                    camera: Camera::new(),
                    tiles: Vec::new(),
                    eyes: Vec::new(),
                }));
            } else if self.selected_option == 1 {
                return Some(Box::new(HelpState {
                    camera: Camera::new(),
                }));
            } else if self.selected_option == 2 {
                return Some(Box::new(CreditsState {
                    camera: Camera::new(),
                }));
            }
        }

        None
    }

    fn draw(&mut self, context: &mut BarnContext, bgfx: &mut BarnGFX) {
        // Clear screen to black.
        bgfx.sdl.set_draw_color(Color::BLACK);
        bgfx.sdl.clear();

        // Get font from cache.
        let font = context.load_font(*settings::FONT_DETAILS);
        bgfx.sdl.set_draw_color(Color::WHITE);

        // Render game title.
        bgfx.sdl.draw_text("Mind's Eye", font, 
            self.camera.width as f32 / 2.0,
            30.0,
            4.0,
            4.0,
            true,
            false);

        // Render selector box.
        let color = if self.selected_option == 0 {
            Color::from_rgb(0, 0, 180)
        } else {
            if self.selected_option == 1 {
                Color::from_rgb(0, 180, 0)
            } else {
                Color::from_rgb(180, 0, 0)
            }
        };
        bgfx.sdl.set_draw_color(color);

        bgfx.sdl.draw_rect(
            self.camera.width / 2 - 125, 
            200 + self.selected_option as i32 * 100, 
            250, 
            65,
            FillType::FILL,
            false);

        // Draw option border.
        bgfx.sdl.set_draw_color(Color::from_rgb(255, 255, 255));
        bgfx.sdl.draw_rect(self.camera.width / 2 - 125, 200, 250, 65, FillType::LINE, false);
        bgfx.sdl.draw_rect(self.camera.width / 2 - 125, 300, 250, 65, FillType::LINE, false);
        bgfx.sdl.draw_rect(self.camera.width / 2 - 125, 400, 250, 65, FillType::LINE, false);
       
        // Render the options.
        bgfx.sdl.set_draw_color(Color::WHITE);
        bgfx.sdl.draw_text("Play", font, 
            self.camera.width as f32 / 2.0,
            200.0,
            3.0,
            3.0,
            true,
            false);

        bgfx.sdl.draw_text("Help", font, 
            self.camera.width as f32 / 2.0,
            300.0,
            3.0,
            3.0,
            true,
            false);

        bgfx.sdl.draw_text("Credits", font, 
            self.camera.width as f32 / 2.0,
            400.0,
            3.0,
            3.0,
            true,
            false);
        
        // Render menu eyes.
        for eye in self.eyes.iter_mut() {
            let socket_tex =  context.load_texture(String::from("res/img/socket.png"));
            eye.draw_socket(socket_tex, &mut self.camera, bgfx);
            let tex_pupil = if eye.color == "blue" {
                context.load_texture(String::from("res/img/bluepupil.png"))
            } else {
                if eye.color == "red" {
                    context.load_texture(String::from("res/img/redpupil.png"))
                } else {
                    context.load_texture(String::from("res/img/greenpupil.png"))
                }
            };
            eye.draw_iris(tex_pupil, &mut self.camera, bgfx);
        }
        
        bgfx.sdl.present();
    }

    fn on_enter(&mut self, context: &mut BarnContext) {
        self.camera.width = 800;
        self.camera.height = 600;
        context.load_texture(String::from("res/img/socket.png"));
        context.load_texture(String::from("res/img/bluepupil.png"));
        context.load_texture(String::from("res/img/redpupil.png"));
        context.load_texture(String::from("res/img/greenpupil.png"));
        self.eyes.push(Eye {
            direction: String::from("left"),
            x: 150,
            y: 200,
            width: 64,
            height: 64,
            color: String::from("blue"),
            solved: false,
            deltax: 0.0,
            deltay: 0.0,
            anger: 0,
        });
        self.eyes.push(Eye {
            direction: String::from("left"),
            x: 600,
            y: 300,
            width: 64,
            height: 64,
            color: String::from("green"),
            solved: false,
            deltax: 0.0,
            deltay: 0.0,
            anger: 0,
        });
        self.eyes.push(Eye {
            direction: String::from("left"),
            x: self.camera.width / 2 - 32,
            y: 500,
            width: 64,
            height: 64,
            color: String::from("red"),
            solved: false,
            deltax: 0.0,
            deltay: 0.0,
            anger: 0,
        });
        if self.selected_option == 0 {
            self.tiles.push(Tile {
                texture: String::from("res/img/blueblock.png"),
                bb: BoundingBox2D {origin: Vector2 {x: 200.0, y: 200.0}, width: 64, height: 64},
                target_pos: Vector2 {x: 200.0, y: 200.0},
                resistance: 30.0,
                iswall: false,
                isblock: true,
                moving: false, 
            });
        } else if self.selected_option == 1 {
            self.tiles.push(Tile {
                texture: String::from("res/img/greenblock.png"),
                bb: BoundingBox2D {origin: Vector2 {x: 200.0, y: 300.0}, width: 64, height: 64},
                target_pos: Vector2 {x: 200.0, y: 500.0},
                resistance: 30.0,
                iswall: false,
                isblock: true,
                moving: false, 
            });
        } else if self.selected_option == 2 {
            self.tiles.push(Tile {
                texture: String::from("res/img/redblock.png"),
                bb: BoundingBox2D {origin: Vector2 {x: (self.camera.width / 2 - 32) as f32, y: 300.0}, width: 64, height: 64},
                target_pos: Vector2 {x: 200.0, y: 200.0},
                resistance: 30.0,
                iswall: false,
                isblock: true,
                moving: false, 
            });
        }
        
        self.camera.x = 0;
        self.camera.y = 0;
        //let music = context.load_sound(String::from("res/sound/laidback.mp3"));
        //let _ = load_music(String::from("res/sound/laidback.wav"));

    }

    fn on_exit(&mut self, context: &mut BarnContext) {
        self.eyes.clear();
        self.tiles.clear();
    }

    fn get_name(&mut self) -> String {
        String::from("start_menu")
    }
}
