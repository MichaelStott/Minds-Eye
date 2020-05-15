use crate::context::TILE_HEIGHT;
use crate::context::TILE_WIDTH;
use crate::eye::Eye;
use crate::game_state::GameState;
use crate::context::Context;
use crate::state::State;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

pub struct StartMenuState {
    pub selected_option: u32,
}

impl State for StartMenuState {
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> {
        for eye in context.eyes.iter_mut() {
            eye.update(&mut context.tiles);
        }

        // TODO: Menu options should be selected via arrow keys.
        if context.input.key_pressed(&Keycode::A) {
            Some(Box::new(GameState {}))
        } else {
            None
        }
    }

    /**
     * TODO: Implement better string rendering logic.
     */
    fn draw(&mut self, context: &mut Context, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let font = context.font_manager.load(&context.font_details).unwrap();
        let texture_creator = canvas.texture_creator();

        // Render the title.
        let title = font.render("Mind's Eye").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let title_tex = texture_creator.create_texture_from_surface(&title).unwrap();
        canvas.copy(&title_tex, None, Rect::new(context.camera.width / 2 - 200, 30,400,76)).unwrap();

        // Render the options.
        let title = font.render("Play").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let title_tex = texture_creator.create_texture_from_surface(&title).unwrap();
        canvas.copy(&title_tex, None, Rect::new(context.camera.width / 2 - 50, 200, 100, 57)).unwrap();

        // Render the options.
        let title = font.render("Help").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let title_tex = texture_creator.create_texture_from_surface(&title).unwrap();
        canvas.copy(&title_tex, None, Rect::new(context.camera.width / 2 - 45, 300, 100, 57)).unwrap();

        let title = font.render("Credits").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let title_tex = texture_creator.create_texture_from_surface(&title).unwrap();
        canvas.copy(&title_tex, None, Rect::new(context.camera.width / 2 - 75, 400, 165, 57)).unwrap();

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
    }

    fn on_exit(&mut self, context: &mut Context) {
        context.eyes.clear();
    }

    fn get_name(&mut self) -> String {
        String::from("start")
    }
}
