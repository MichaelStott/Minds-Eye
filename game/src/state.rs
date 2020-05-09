use crate::context::Context;

use sdl2::render::WindowCanvas;

pub trait State {
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>>;

    fn draw(&mut self, context: &mut Context, canvas: &mut WindowCanvas);

    fn on_enter(&mut self);

    fn on_exit(&mut self);

    fn get_name(&mut self) -> String;
}
