use barn::graphics::barn_gfx::BarnGFX;
use barn::game::barn_context::BarnContext;
use barn::graphics::color::Color;
use barn::graphics::fill_type::FillType;
use barn::game::state::State;
use crate::game::camera::Camera;
use crate::game::eye::Eye;
use crate::game::game_state::GameState;
use crate::game::start_menu_state::StartMenuState;
use crate::game::tile::Tile;
use crate::settings;

use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::fs;

pub struct LevelSelectState {
    pub levels: HashMap<String, String>,
    pub options: Vec<String>,
    pub selected_option: i32,
    pub camera: Camera,
    pub tiles: Vec<Tile>,
    pub eyes: Vec<Eye>
}

impl State<BarnContext> for LevelSelectState {
    fn update(&mut self, context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> {
        let prev_option = self.selected_option;
        if context.input.key_just_pressed(&Keycode::Down) {
            if self.selected_option == 4 {
                self.selected_option = 0;
            } else {
                self.selected_option += 1;
            }
        } else if context.input.key_just_pressed(&Keycode::Up) {
            if self.selected_option <= 0 {
                self.selected_option = 4;
            } else {
                self.selected_option -= 1;
            }
        } else if context.input.key_just_pressed(&Keycode::Left) {
            self.selected_option = -1;
        } else if context.input.key_just_pressed(&Keycode::Right) && self.selected_option == -1 {
            self.selected_option = 0;
        } else if context.input.key_just_pressed(&Keycode::B) {
            return Some(Box::new(StartMenuState::new(0)));
        }
        if prev_option != self.selected_option && settings::ENABLE_SOUND {
            let select_fx = context.load_sound(String::from("res/sound/select.ogg"));
            let channel = sdl2::mixer::Channel(1);
            channel.play(select_fx, 0);
        }
        if context.input.key_just_pressed(&Keycode::Return) {
            if self.selected_option == -1 {
                if settings::ENABLE_SOUND {
                    let back_fx = context.load_sound(String::from("res/sound/back.ogg"));
                    let channel = sdl2::mixer::Channel(2);
                    channel.play(back_fx, 0);
                }
                return Some(Box::new(StartMenuState::new(0)));
            } else {
                if settings::ENABLE_SOUND {
                    let enter_fx = context.load_sound(String::from("res/sound/enter.ogg"));
                    let channel = sdl2::mixer::Channel(2);
                    channel.play(enter_fx, 0);
                }
                let key = self.options[self.selected_option as usize].clone();
                let path = self.levels.get(&key).unwrap();
                return Some(Box::new(GameState::new(path.to_string())));
            }
        }
        None
    }

    fn draw(&mut self, context: &mut BarnContext, bgfx: &mut BarnGFX) {
        self.camera.width = 800;
        self.camera.height = 600;

        // Clear screen to black.
        bgfx.sdl.set_draw_color(Color::BLACK);
        bgfx.sdl.clear();

        // Get font from cache.
        let font = context.load_font(*settings::FONT_DETAILS);
        bgfx.sdl.set_draw_color(Color::WHITE);

        // Render screen title.
        bgfx.sdl.draw_text("Level Select", font, 
            self.camera.width as f32 / 2.0,
            30.0,
            4.0,
            4.0,
            true,
            false);


        let mut counter: i32 = 0;
        for level in self.options.iter_mut() {
            let color = if self.selected_option == counter {
                Color::from_rgb(0, 0, 0)
            } else {
                Color::from_rgb(255, 255, 255)
            };

            // let level = font.render(&level).blended(color).unwrap();
            // let level_tex = texture_creator.create_texture_from_surface(&level).unwrap();
            if self.selected_option == counter {
                bgfx.sdl.set_draw_color(Color::WHITE);
                bgfx.sdl.draw_rect(
                    self.camera.width / 2 - 250,
                    200 + 50 * (counter -1), 
                    500, 
                    50, 
                    FillType::FILL, 
                    false);
            }
            bgfx.sdl.set_draw_color(color);
            bgfx.sdl.draw_text(&level, font, 
                (self.camera.width / 2) as f32,
                (200 + 50 * (counter - 1)) as f32 + 25.0,
                2.0,
                2.0,
                true,
                true);
            bgfx.sdl.set_draw_color(color);
            bgfx.sdl.draw_rect(
                self.camera.width / 2 - 250,
                200 + 50 * (counter -1), 
                500, 
                50, 
                FillType::LINE, 
                false);

            counter += 1;
        }

        if self.selected_option == -1 {
            bgfx.sdl.set_draw_color(Color::from_rgb(255, 255, 255));
            bgfx.sdl.draw_rect(
                0,
                self.camera.height - 50, 
                100, 
                50, 
                FillType::FILL, 
                false);
        }
        let color = if self.selected_option == -1 {
            Color::from_rgb(0, 0, 0)
        } else {
            Color::from_rgb(255, 255, 255)
        };
        bgfx.sdl.set_draw_color(color);
        bgfx.sdl.draw_text("< Back", font, 
            0.0,
            (self.camera.height - 50) as f32,
            2.0,
            2.0,
            false,
            false);
        bgfx.sdl.present();
    }

    fn on_enter(&mut self, context: &mut BarnContext) {
        let paths = fs::read_dir("./res/levels/").unwrap();
        self.camera = Camera::new();
        for path in paths {
            let path_str = path.unwrap().path().to_str().unwrap().to_string();
            let f = fs::read_to_string(&path_str).expect("Could not load level!");
            self.levels.insert(
                f.lines()
                    .enumerate()
                    .filter(|&(i, _)| i == 0)
                    .map(|(_, e)| e)
                    .next()
                    .unwrap()
                    .to_string(),
                path_str,
            );
        }
        for key in self.levels.keys() {
            self.options.push(key.to_string());
        }
        self.options.sort();
    }

    fn on_exit(&mut self, context: &mut BarnContext) {
        self.eyes.clear();
        self.tiles.clear();
        self.levels.clear();
    }

    fn get_name(&mut self) -> String {
        String::from("level_select")
    }
}

impl LevelSelectState {
    pub fn new(option: i32) -> Self {
        LevelSelectState {
            levels: HashMap::new(),
            options: Vec::new(),
            selected_option: option,
            camera: Camera::new(),
            tiles: Vec::new(),
            eyes: Vec::new()
        }
    }
}
