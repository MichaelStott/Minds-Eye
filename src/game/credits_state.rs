use barn::game::context::Context;
use barn::game::state::State;
use crate::game::camera::Camera;
use crate::game::start_menu_state::StartMenuState;
use crate::settings;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct CreditsState {
    pub camera: Camera,
}

impl State for CreditsState {
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> {
        if context.input.key_just_pressed(&Keycode::Return) {
            let back_fx = context.load_sound(String::from("res/sound/back.ogg"));
            let channel = sdl2::mixer::Channel(2);
            channel.play(back_fx, 0).unwrap();
            return Some(Box::new(StartMenuState::new(2)));
        }
        None
    } 

    fn draw(&mut self, context: &mut Context, canvas: &mut WindowCanvas) {
        self.camera.width = (canvas.output_size().unwrap().0) as i32;
        self.camera.height = (canvas.output_size().unwrap().1) as i32;
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let font = context.load_font(*settings::FONT_DETAILS);
        let texture_creator = canvas.texture_creator();

        let title = font
            .render("Credits")
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let title_tex = texture_creator.create_texture_from_surface(&title).unwrap();
        canvas
            .copy(
                &title_tex,
                None,
                Rect::new(
                    self.camera.width / 2 - 4 * title.size().0 as i32 / 2,
                    30,
                    title.size().0 * 4,
                    title.size().1 * 4,
                ),
            )
            .unwrap();

        let play = font
            .render("Developed by Michael Stott")
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let play_tex = texture_creator.create_texture_from_surface(&play).unwrap();
        canvas
            .copy(
                &play_tex,
                None,
                Rect::new(
                    self.camera.width / 2 - play.size().0 as i32 * 3 / 2,
                    200,
                    play.size().0 * 3,
                    play.size().1 * 3,
                ),
            )
            .unwrap();

        let help = font
            .render("Music by Kevin MacLeod")
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let help_tex = texture_creator.create_texture_from_surface(&help).unwrap();
        canvas
            .copy(
                &help_tex,
                None,
                Rect::new(
                    self.camera.width / 2 - help.size().0 as i32 * 3 / 2,
                    300,
                    help.size().0 * 3,
                    help.size().1 * 3,
                ),
            )
            .unwrap();

        // Render number of moves.
        let back = font
            .render("Press enter to return to the menu.")
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let back_tex = texture_creator.create_texture_from_surface(&back).unwrap();
        canvas
            .copy(
                &back_tex,
                None,
                Rect::new(
                    0,
                    self.camera.height - back.size().1 as i32 * 2,
                    back.size().0 * 2,
                    back.size().1 * 2,
                ),
            )
            .unwrap();

        canvas.present();
    }

    fn on_enter(&mut self, context: &mut Context) {
        self.camera = Camera::new();
    }

    fn on_exit(&mut self, context: &mut Context) {}

    fn get_name(&mut self) -> String {
        String::from("credits")
    }
}
