use crate::context::Context;
use crate::physics::handle_collisions;
use crate::state::State;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::keyboard::Keycode;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct GameState {
    
}

impl State for GameState {
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> {
        if context.input.key_just_pressed(&Keycode::R) {
            context.load_level(String::from("res/levels/level2.txt"));
        }

        // TODO: Refactor this.
        let new_tiles = &mut context.tiles.to_vec();
        for tile in context.tiles.iter_mut() {
            tile.update(new_tiles, &context.move_fx);
        }

        // Check if the puzzle has been solved.
        let mut have_won = true;
        for eye in context.eyes.iter_mut() {
            eye.update(&mut context.tiles);
            if !eye.solved {
                have_won = false;
            }
        }
        if have_won {
            // Return a state, perform some action, etc. etc.
            //std::process::exit(0);
        }

        // Update the player.
        context.player.update(&mut context.input);
        handle_collisions(&mut context.player, &mut context.tiles, &context.move_fx);
        context.camera.focus(context.player.x + context.player.width as i32 / 2, context.player.y + context.player.height as i32 / 2);

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
        canvas.present();
    }
    
    fn on_enter(&mut self, context: &mut Context) {
        context.load_level(String::from("res/levels/level5.txt"));
    }

    fn on_exit(&mut self, context: &mut Context) {}

    fn get_name(&mut self) -> String {
        String::from("game")
    }
}
