use barn::graphics::barn_gfx::BarnGFX;
use barn::fonts::font_details::FontDetails;
use barn::game::barn_context::BarnContext;
use barn::game::context::Context;
use barn::game::state::State;
use barn::graphics::color::Color;
use crate::game::camera::Camera;
use crate::game::start_menu_state::StartMenuState;
use crate::settings;

use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct HelpState {
    pub camera: Camera,
}

impl State<BarnContext> for HelpState {
    fn update(&mut self, context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> {
        if context.input.key_just_pressed(&Keycode::Return) {
            if (settings::ENABLE_SOUND) {
                let back_fx = context.load_sound(String::from("res/sound/back.ogg"));
                let channel = sdl2::mixer::Channel(2);
                channel.play(back_fx, 0).unwrap();
            }
            
            return Some(Box::new(StartMenuState::new(1)));
        }
        None
    }

    fn draw(&mut self, context: &mut BarnContext, bgfx: &mut BarnGFX) {
        // Clear screen to black.
        bgfx.sdl.set_draw_color(Color::BLACK);
        bgfx.sdl.clear();

        // Get font from cache.
        let font = context.load_font(*settings::FONT_DETAILS);
        bgfx.sdl.set_draw_color(Color::WHITE);

        // Render instructions
        bgfx.sdl.draw_text("Help", font, 
            self.camera.width as f32 / 2.0,
            30.0,
            4.0,
            4.0,
            true,
            false);

        bgfx.sdl.draw_text("Movement: Arrow Keys", font, 
            self.camera.width as f32 / 2.0,
            200.0,
            3.0,
            3.0,
            true,
            false);

        bgfx.sdl.draw_text("Reset Puzzle: R", font, 
            self.camera.width as f32 / 2.0,
            300.0,
            3.0,
            3.0,
            true,
            false);

        bgfx.sdl.draw_text("Exit Puzzle: Q", font, 
            self.camera.width as f32 / 2.0,
            400.0,
            3.0,
            3.0,
            true,
            false);

        bgfx.sdl.draw_text("Press enter to return to the menu.", font, 
            0.0,
            550.0,
            2.0,
            2.0,
            false,
            false);       

        bgfx.sdl.present();
    }

    fn on_enter(&mut self, context: &mut BarnContext) {
        self.camera = Camera::new();
        self.camera.width = 800;
        self.camera.height = 600;
    }

    fn on_exit(&mut self, context: &mut BarnContext) {}

    fn get_name(&mut self) -> String {
        String::from("help")
    }
}

impl HelpState {
    fn new() -> Self {
        HelpState {
            camera: Camera::new()
        }
    }
}