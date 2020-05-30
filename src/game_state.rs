use crate::level_select_state::LevelSelectState;
use crate::context::Context;
use crate::physics::handle_collisions;
use crate::state::State;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone)]
pub struct GameState {
    pub level_path: String,
    pub won: bool,
    pub time: Instant,
    pub time_str: String,
    pub moves: u32
}

impl State for GameState {
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> {
        if context.input.key_just_pressed(&Keycode::R) {
            context.load_level(String::from(&self.level_path));
        } else if context.input.key_just_pressed(&Keycode::Q) {
            return Some(Box::new(LevelSelectState {levels: HashMap::new(), options: Vec::new(), selected_option: 0}))
        }

        // TODO: Refactor this.
        let new_tiles = &mut context.tiles.to_vec();
        for tile in context.tiles.iter_mut() {
            if tile.has_moved() {
                self.moves += 1;
            }
            tile.update(new_tiles, &context.move_fx);
        }

        // Check if the puzzle has been solved.
        self.won = true;
        for eye in context.eyes.iter_mut() {
            eye.update(&mut context.tiles);
            if !eye.solved {
                self.won = false;
            }
        }
        if !self.won {
             // Update the player.
            context.player.update(&mut context.input);
            handle_collisions(&mut context.player, &mut context.tiles, &context.move_fx);
            context.camera.focus(context.player.x + context.player.width as i32 / 2, context.player.y + context.player.height as i32 / 2);
        } else {
            if self.time_str == "" {
                let elapsed_time = self.time.elapsed().as_secs();
                self.time_str = elapsed_time.to_string();
            }
            if context.input.key_just_pressed(&Keycode::Return) {
                return Some(Box::new(LevelSelectState {levels: HashMap::new(), options: Vec::new(), selected_option: 0}))
            }
        } 

        // No state change has occured.
        None
    }

    fn draw(&mut self, context: &mut Context, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Get all of the image assets.
        let tex_player = context.texture_manager.load("res/img/player.png").unwrap();
        let tex_shadow = context
            .texture_manager
            .load("res/img/drop_shadow.png")
            .unwrap();
        // Draw the scene.
        for tile in &mut context.tiles {
            if !tile.isblock && !tile.iswall {
                tile.draw(
                    &context.texture_manager.load(&tile.texture).unwrap(),
                    &mut context.camera,
                    canvas,
                );
            }
        }
        context
            .player
            .draw_shadow(&tex_shadow, &mut context.camera, canvas);
        for tile in &mut context.tiles {
            if tile.isblock || tile.iswall {
                tile.draw(
                    &context.texture_manager.load(&tile.texture).unwrap(),
                    &mut context.camera,
                    canvas,
                );
            }
        }
        for eye in context.eyes.iter_mut() {
            let tex_pupil = if eye.color == "blue" {
                context
                    .texture_manager
                    .load("res/img/bluepupil.png")
                    .unwrap()
            } else {
                if eye.color == "red" {
                    context
                        .texture_manager
                        .load("res/img/redpupil.png")
                        .unwrap()
                } else {
                    context
                        .texture_manager
                        .load("res/img/greenpupil.png")
                        .unwrap()
                }
            };
            eye.draw(
                &mut context.socket_tex,
                &tex_pupil,
                &mut context.camera,
                canvas,
            );
        }
        context.player.draw(&tex_player, &mut context.camera, canvas);

        if self.won {
            canvas.set_draw_color(Color::RGBA(0, 0, 0, 150));
            canvas.set_blend_mode(BlendMode::Blend);
            canvas.fill_rect(Rect::new(0, 0, context.camera.width as u32, context.camera.height as u32)).unwrap();
            let font = context.font_manager.load(&context.font_details).unwrap();
            let texture_creator = canvas.texture_creator();

            // Render the title.
            let title = font.render("Solved!").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
            let title_tex = texture_creator.create_texture_from_surface(&title).unwrap();
            canvas.copy(&title_tex, None, Rect::new(context.camera.width / 2 - 4 * title.size().0 as i32 / 2, 30, title.size().0 * 4,  title.size().1 * 4)).unwrap();

            // Render time result.
            let time = font.render(&format!("Time: {} seconds", self.time_str)).blended(Color::RGBA(255, 255, 255, 255)).unwrap();
            let time_tex = texture_creator.create_texture_from_surface(&time).unwrap();
            canvas.copy(&time_tex, None, Rect::new(context.camera.width / 2 - 3 * time.size().0 as i32 / 2, context.camera.height / 2 - 3 *  time.size().1 as i32 / 2, time.size().0 * 3,  time.size().1 * 3)).unwrap();

            // Render number of moves.
            let moves = font.render(&format!("Moves taken: {}", self.moves)).blended(Color::RGBA(255, 255, 255, 255)).unwrap();
            let moves_tex = texture_creator.create_texture_from_surface(&moves).unwrap();
            canvas.copy(&moves_tex, None, Rect::new(context.camera.width / 2 - 3 * moves.size().0 as i32 / 2, context.camera.height / 2 + 4 *  moves.size().1 as i32 / 2, moves.size().0 * 3,  moves.size().1 * 3)).unwrap();

             // Render number of moves.
             let back = font.render("Press enter to go back").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
             let back_tex = texture_creator.create_texture_from_surface(&back).unwrap();
             canvas.copy(&back_tex, None, Rect::new(0, context.camera.height -  back.size().1 as i32 * 2, back.size().0 * 2,  back.size().1 * 2)).unwrap();
        }
        canvas.present();
    }
    
    fn on_enter(&mut self, context: &mut Context) {
        context.load_level(String::from(&self.level_path));
        self.time = Instant::now();
    }

    fn on_exit(&mut self, context: &mut Context) {}

    fn get_name(&mut self) -> String {
        String::from("game")
    }
}
