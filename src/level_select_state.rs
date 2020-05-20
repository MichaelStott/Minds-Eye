
use std::collections::HashMap;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::path::Path;
use std::fs;
use sdl2::pixels::Color;
use crate::context::Context;
use crate::state::State;

pub struct LevelSelectState {
    pub levels: HashMap<String, String>,
    pub options: Vec<String>
}

impl State for LevelSelectState {

    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> {
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

        let mut counter = 0;
        
        for level in self.options.iter_mut() {
            counter += 1;
            let level = font.render(&level).blended(Color::RGBA(255, 255, 255, 255)).unwrap();
            let level_tex = texture_creator.create_texture_from_surface(&level).unwrap();
            canvas.copy(&level_tex, None, Rect::new(context.camera.width / 2 - level.size().0 as i32 * 2 / 2, 200 + 50 * (counter - 1),  level.size().0 * 2,  level.size().1 * 2)).unwrap();
        }
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