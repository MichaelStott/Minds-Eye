use crate::start_menu_state::StartMenuState;
use crate::context::Context;
use crate::state::State;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

#[derive(Clone)]
pub struct CreditsState {
}

impl State for CreditsState {
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> {
        if context.input.key_just_pressed(&Keycode::Return) {
            let channel = sdl2::mixer::channel(2);
            channel.play(&context.back_fx, 0);
            return Some(Box::new(StartMenuState { selected_option: 2 }))
        }
        None
    }
    
    fn draw(&mut self, context: &mut Context, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let font = context.font_manager.load(&context.font_details).unwrap();
        let texture_creator = canvas.texture_creator();

        
        let title = font.render("Credits").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let title_tex = texture_creator.create_texture_from_surface(&title).unwrap();
        canvas.copy(&title_tex, None, Rect::new(context.camera.width / 2 - 4 * title.size().0 as i32 / 2, 30, title.size().0 * 4,  title.size().1 * 4)).unwrap();

        let play = font.render("Developed by Michael Stott").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let play_tex = texture_creator.create_texture_from_surface(&play).unwrap();
        canvas.copy(&play_tex, None, Rect::new(context.camera.width / 2 - play.size().0 as i32 * 3 / 2, 200,  play.size().0 * 3,  play.size().1 * 3)).unwrap();

        let help = font.render("Music by Kevin MacLeod").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let help_tex = texture_creator.create_texture_from_surface(&help).unwrap();
        canvas.copy(&help_tex, None, Rect::new(context.camera.width / 2 - help.size().0 as i32 * 3 / 2, 300,  help.size().0 * 3,  help.size().1 * 3)).unwrap();

        // Render number of moves.
        let back = font.render("Press enter to return to the menu.").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let back_tex = texture_creator.create_texture_from_surface(&back).unwrap();
        canvas.copy(&back_tex, None, Rect::new(0, context.camera.height -  back.size().1 as i32 * 2, back.size().0 * 2,  back.size().1 * 2)).unwrap();

        canvas.present();
    }
    
    fn on_enter(&mut self, context: &mut Context) {

    }
    
    fn on_exit(&mut self, context: &mut Context) {

    }

    fn get_name(&mut self) -> String {
        String::from("credits")
    }
}