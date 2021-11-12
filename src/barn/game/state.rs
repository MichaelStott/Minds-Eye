
use sdl2::render::TextureCreator;
use crate::barn::game::context::Context;

use sdl2::render::WindowCanvas;

pub trait State {
    // Update game logic.
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>>;

    // Render the game entities.
    fn draw(&mut self, context: &mut Context, canvas: &mut WindowCanvas);

    // Perform any initialization here.
    fn on_enter(&mut self, context: &mut Context);

    // Perform any cleanup before transitioning to the next state.
    fn on_exit(&mut self, context: &mut Context);

    // Get the state name.
    fn get_name(&mut self) -> String;
}
