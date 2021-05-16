use crate::credits_state::CreditsState;
use crate::help_state::HelpState;
use crate::level_select_state::LevelSelectState;
use crate::tile::Tile;
use crate::context::TILE_HEIGHT;
use crate::context::TILE_WIDTH;
use crate::eye::Eye;
use crate::game_state::GameState;
use crate::context::Context;
use crate::barn::game::state::State;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use std::collections::HashMap;

pub struct StartMenuState {
    pub selected_option: u32,
}

impl State for StartMenuState {
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> {
        for eye in context.eyes.iter_mut() {
            eye.update(&mut context.tiles);
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
            let channel = sdl2::mixer::channel(1);
            channel.play(&context.select_fx, 0);
            context.tiles.clear();
            if self.selected_option == 0 {
                context.tiles.push(Tile {
                    texture: String::from("res/img/blueblock.png"),
                    width: TILE_WIDTH,
                    height: TILE_HEIGHT,
                    x: 200,
                    y: 200,
                    targetx: 200,
                    targety: 200,
                    resistancex: 30,
                    resistancey: 30,
                    iswall: false,
                    isblock: true,
                });
            } else if self.selected_option == 1 {
                context.tiles.push(Tile {
                    texture: String::from("res/img/greenblock.png"),
                    width: TILE_WIDTH,
                    height: TILE_HEIGHT,
                    x: 200,
                    y: 300,
                    targetx: 200,
                    targety: 500,
                    resistancex: 30,
                    resistancey: 30,
                    iswall: false,
                    isblock: true,
                });
            } else if self.selected_option == 2 {
                context.tiles.push(Tile {
                    texture: String::from("res/img/redblock.png"),
                    width: TILE_WIDTH,
                    height: TILE_HEIGHT,
                    x: context.camera.width / 2 - 32,
                    y: 200,
                    targetx: 200,
                    targety: 200,
                    resistancex: 30,
                    resistancey: 30,
                    iswall: false,
                    isblock: true,
                });
            }
        }

        if context.input.key_just_pressed(&Keycode::Return) {
            let channel = sdl2::mixer::channel(2);
            channel.play(&context.enter_fx, 0);
            if self.selected_option == 0 {
               return Some(Box::new(LevelSelectState {levels: HashMap::new(), options: Vec::new(), selected_option: 0}))
            } else if self.selected_option == 1 {
                return Some(Box::new(HelpState {}))
            } else if self.selected_option == 2 {
                return Some(Box::new(CreditsState {}))
            }
        }

        None
    }

    /**
     * TODO: Implement better string rendering logic. Below will create a new
     * texture for the strings every frame.
     */
    fn draw(&mut self, context: &mut Context, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let font = context.font_manager.load(&context.font_details).unwrap();
        let texture_creator = canvas.texture_creator();

        // Render the title.
        let title = font.render("Mind's Eye").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let title_tex = texture_creator.create_texture_from_surface(&title).unwrap();
        canvas.copy(&title_tex, None, Rect::new(context.camera.width / 2 - 4 * title.size().0 as i32 / 2, 30, title.size().0 * 4,  title.size().1 * 4)).unwrap();

         // Render selector box.
        let color = if self.selected_option == 0 {
            Color::RGB(0, 0, 180)
        } else {
            if self.selected_option == 1 {
                Color::RGB(0, 180, 0)
            } else {
                Color::RGB(180, 0, 0)
            }
        };
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(
                context.camera.width / 2 - 125,
                200 + self.selected_option as i32 * 100,
                250,
                65,
            )).unwrap();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rect(Rect::new(
                context.camera.width / 2 - 125,
                200,
                250,
                65,
            )).unwrap();
        canvas.draw_rect(Rect::new(
                context.camera.width / 2 - 125,
                300,
                250,
                65,
            )).unwrap();
        canvas.draw_rect(Rect::new(
                context.camera.width / 2 - 125,
                400,
                250,
                65,
            )).unwrap();

        // Render the options.
        let play = font.render("Play").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let play_tex = texture_creator.create_texture_from_surface(&play).unwrap();
        canvas.copy(&play_tex, None, Rect::new(context.camera.width / 2 - play.size().0 as i32 * 3 / 2, 200,  play.size().0 * 3,  play.size().1 * 3)).unwrap();

        let help = font.render("Help").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let help_tex = texture_creator.create_texture_from_surface(&help).unwrap();
        canvas.copy(&help_tex, None, Rect::new(context.camera.width / 2 - help.size().0 as i32 * 3 / 2, 300,  help.size().0 * 3,  help.size().1 * 3)).unwrap();

        let credits = font.render("Credits").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let credits_tex = texture_creator.create_texture_from_surface(&credits).unwrap();
        canvas.copy(&credits_tex, None, Rect::new(context.camera.width / 2 - credits.size().0 as i32 * 3 / 2, 400,  credits.size().0 * 3,  credits.size().1 * 3)).unwrap();

        // Render menu eyes.
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
        canvas.present();
    }

    fn on_enter(&mut self, context: &mut Context) {
        context.eyes.push(Eye {
            direction: String::from("left"),
            x: 150,
            y: 200,
            width: TILE_WIDTH,
            height: TILE_HEIGHT,
            color: String::from("blue"),
            solved: false,
            deltax: 0,
            deltay: 0,
            anger: 0,
        });
        context.eyes.push(Eye {
            direction: String::from("left"),
            x: 600,
            y: 300,
            width: TILE_WIDTH,
            height: TILE_HEIGHT,
            color: String::from("green"),
            solved: false,
            deltax: 0,
            deltay: 0,
            anger: 0,
        });
        context.eyes.push(Eye {
            direction: String::from("left"),
            x: context.camera.width / 2 - 32,
            y: 500,
            width: TILE_WIDTH,
            height: TILE_HEIGHT,
            color: String::from("red"),
            solved: false,
            deltax: 0,
            deltay: 0,
            anger: 0,
        });
        if self.selected_option == 0 {
            context.tiles.push(Tile {
                texture: String::from("res/img/blueblock.png"),
                width: TILE_WIDTH,
                height: TILE_HEIGHT,
                x: 200,
                y: 200,
                targetx: 200,
                targety: 200,
                resistancex: 30,
                resistancey: 30,
                iswall: false,
                isblock: true,
            });
        } else if self.selected_option == 1 {
            context.tiles.push(Tile {
                texture: String::from("res/img/greenblock.png"),
                width: TILE_WIDTH,
                height: TILE_HEIGHT,
                x: 200,
                y: 300,
                targetx: 200,
                targety: 500,
                resistancex: 30,
                resistancey: 30,
                iswall: false,
                isblock: true,
            });
        } else if self.selected_option == 2 {
            context.tiles.push(Tile {
                texture: String::from("res/img/redblock.png"),
                width: TILE_WIDTH,
                height: TILE_HEIGHT,
                x: context.camera.width / 2 - 32,
                y: 200,
                targetx: 200,
                targety: 200,
                resistancex: 30,
                resistancey: 30,
                iswall: false,
                isblock: true,
            });
        }
        context.camera.x = 0;
        context.camera.y = 0;
    }

    fn on_exit(&mut self, context: &mut Context) {
        context.eyes.clear();
        context.tiles.clear();
    }

    fn get_name(&mut self) -> String {
        String::from("start_menu")
    }
}
