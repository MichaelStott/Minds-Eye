use crate::context::Context;
use crate::physics::handle_collisions;
use crate::state::State;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

#[derive(Clone)]
pub struct GameState {}

impl State for GameState {
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> {
        // TODO: Refactor this.
        let new_tiles = &mut context.tiles.to_vec();
        for tile in context.tiles.iter_mut() {
            tile.update(new_tiles);
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
        handle_collisions(&mut context.player, &mut context.tiles);
        context.camera.focus(context.player.x, context.player.y);

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
        let mut tex_socket = context.texture_manager.load("res/img/socket.png").unwrap();

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
            .draw(&tex_player, &tex_shadow, &mut context.camera, canvas);
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
                &mut tex_socket.clone(),
                &tex_pupil,
                &mut context.camera,
                canvas,
            );
        }
        canvas.present();
    }

    fn on_enter(&mut self) {}

    fn on_exit(&mut self) {}

    fn get_name(&mut self) -> String {
        String::from("game")
    }
}
