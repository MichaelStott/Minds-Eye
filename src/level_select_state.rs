
use crate::game_state::GameState;
use crate::start_menu_state::StartMenuState;
use std::collections::HashMap;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::path::Path;
use std::time::Instant;
use std::fs;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use crate::context::Context;
use crate::state::State;

pub struct LevelSelectState {
    pub levels: HashMap<String, String>,
    pub options: Vec<String>,
    pub selected_option: i32
}

impl State for LevelSelectState {

    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> {
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
            return Some(Box::new(StartMenuState {selected_option: 0}))
        }
        if prev_option != self.selected_option {
            let channel = sdl2::mixer::channel(1);
            channel.play(&context.select_fx, 0);
        }
        if context.input.key_just_pressed(&Keycode::Return) {
            if self.selected_option == -1 {
                let channel = sdl2::mixer::channel(2);
                channel.play(&context.back_fx, 0);
                return Some(Box::new(StartMenuState {selected_option: 0}))
            } else {
                let channel = sdl2::mixer::channel(2);
                channel.play(&context.enter_fx, 0);
                let key = self.options[self.selected_option as usize].clone();
                let path = self.levels.get(&key).unwrap();
                return Some(Box::new(GameState {level_path: path.to_string(), won: false, time: Instant::now(), time_str: String::from(""), moves: 0}))
            }
        }
        None
    }

    fn draw(&mut self, context: &mut Context, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let font = context.font_manager.load(&context.font_details).unwrap();
        let texture_creator = canvas.texture_creator();

        // Render the title.
        let title = font.render("Level Select").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let title_tex = texture_creator.create_texture_from_surface(&title).unwrap();
        canvas.copy(&title_tex, None, Rect::new(context.camera.width / 2 - 4 * title.size().0 as i32 / 2, 30, title.size().0 * 4,  title.size().1 * 4)).unwrap();

        let mut counter: i32 = 0;
        for level in self.options.iter_mut() {
            let color = if self.selected_option == counter {
                Color::RGB(0, 0, 0)
            } else {
                Color::RGB(255, 255, 255)
            };
            let level = font.render(&level).blended(color).unwrap();
            let level_tex = texture_creator.create_texture_from_surface(&level).unwrap();
            if self.selected_option == counter {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.fill_rect(Rect::new(context.camera.width / 2 - 250, 200 + 50 * (counter - 1),  500,  level.size().1 * 2)).unwrap();
            }
            canvas.copy(&level_tex, None, Rect::new(context.camera.width / 2 - level.size().0 as i32 * 2 / 2, 200 + 50 * (counter - 1),  level.size().0 * 2,  level.size().1 * 2)).unwrap();
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.draw_rect(Rect::new(context.camera.width / 2 - 250, 200 + 50 * (counter - 1),  500,  level.size().1 * 2)).unwrap();
            counter += 1;
        }

        let color = if self.selected_option == -1 {
            Color::RGB(0, 0, 0)
        } else {
            Color::RGB(255, 255, 255)
        };
        let back = font.render("< Back").blended(color).unwrap();
        let back_tex = texture_creator.create_texture_from_surface(&back).unwrap();
        if self.selected_option == -1 {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.fill_rect(Rect::new(0, context.camera.height - back.size().1 as i32 * 2,  back.size().0 * 2,  back.size().1 * 2)).unwrap();
        }
        canvas.copy(&back_tex, None, Rect::new(0, context.camera.height - back.size().1 as i32 * 2,  back.size().0 * 2,  back.size().1 * 2)).unwrap();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rect(Rect::new(0, context.camera.height - back.size().1 as i32 * 2,  back.size().0 * 2,  back.size().1 * 2)).unwrap();
        canvas.present();
    }

    fn on_enter(&mut self, context: &mut Context) {
        let paths = fs::read_dir("./res/levels/").unwrap();
        for path in paths {
            let path_str = path.unwrap().path().to_str().unwrap().to_string();
            let f = fs::read_to_string(&path_str).expect("Could not load level!");
            self.levels.insert(f.lines().enumerate().filter(|&(i, _)| i == 0).map(|(_, e)| e).next().unwrap().to_string(), path_str);
        }
        for key in self.levels.keys() {
            self.options.push(key.to_string());
        }
        self.options.sort();
    }

    fn on_exit(&mut self, context: &mut Context) {
        context.eyes.clear();
        context.tiles.clear();
        self.levels.clear();
    }

    fn get_name(&mut self) -> String {
        String::from("level_select")
    }
}