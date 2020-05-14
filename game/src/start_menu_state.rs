use crate::context::Context;
use crate::state::State;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct StartMenuState {
    pub selected_option: u32,
}

impl State for StartMenuState {
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> {
        // // TODO: Put this in a utility function...
        // let mut keys: HashSet<Keycode> = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        // // TODO: Menu options should be selected via arrow keys.
        // if keys.contains(&Keycode::A) {
        //     Some(Box::new(GameState {}))
        // } else {
        //     None
        // }
        None
    }

    fn draw(&mut self, context: &mut Context, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        let font = context.font_manager.load(&context.font_details).unwrap();
        let surface = font.render("Hello Rust!").blended(Color::RGBA(255, 0, 0, 255)).unwrap();
        //let font_texture = context.texture_manager.texture_creator.create_texture_from_surface(&surface).unwrap();
        //canvas.copy(&font_texture, None, None)?;
        canvas.present();
    }

    fn on_enter(&mut self) {}

    fn on_exit(&mut self) {}

    fn get_name(&mut self) -> String {
        String::from("start")
    }
}
