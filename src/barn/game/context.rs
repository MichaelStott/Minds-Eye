use crate::barn::game::state::State;
use crate::barn::input::keyboard_handler::KeyboardHandler;

use sdl2::EventPump;
use sdl2::render::WindowCanvas;

// pub trait Context {


//     fn get_input_handler(&mut self) -> &'static mut KeyboardHandler;

//     fn update(
//         &mut self,
//         state: &mut dyn State,
//         event: &mut EventPump,
//     ) -> Option<Box<dyn State>> {
//         self.get_input_handler().update(event);
//         state.update(self);
//         None
//     }

//     fn draw(&mut self, state: &mut dyn State, canvas: &mut WindowCanvas) {
//         //state.draw(self, canvas);
//         self.get_input_handler().refresh_prev();
//     }
// }