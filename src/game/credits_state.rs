use barn::graphics::barn_gfx::BarnGFX;
use barn::game::barn_context::BarnContext;
use barn::game::state::State;
use barn::graphics::color::Color;
use barn::input::SdlKeycode;
use crate::game::camera::Camera;
use crate::game::start_menu_state::StartMenuState;
use crate::settings;

pub struct CreditsState {
    pub camera: Camera,
}

impl State<BarnContext> for CreditsState {
    fn update(&mut self, context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> {
        if context.input.key_just_pressed(&SdlKeycode::Return) {
            if settings::ENABLE_SOUND {
                let back_fx = context.load_sound(String::from("res/sound/back.ogg"));
                let channel = sdl2::mixer::Channel(2);
                channel.play(back_fx, 0).unwrap();
            }
            
            return Some(Box::new(StartMenuState::new(2)));
        }
        None
    } 

    fn draw(&mut self, context: &mut BarnContext, bgfx: &mut BarnGFX) {

        bgfx.sdl.set_draw_color(Color::BLACK);
        bgfx.sdl.clear();

        let font = context.load_font(*settings::FONT_DETAILS);
        bgfx.sdl.set_draw_color(Color::WHITE);
        
        bgfx.sdl.draw_text("Credits", font, 
            self.camera.width as f32 / 2.0,
            30.0,
            4.0,
            4.0,
            true,
            false);

        bgfx.sdl.draw_text("Developed by yam-head", font, 
            self.camera.width as f32 / 2.0,
            200.0,
            3.0,
            3.0,
            true,
            false);

        bgfx.sdl.draw_text("Music by Kevin MacLeod", font, 
            self.camera.width as f32 / 2.0,
            300.0,
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

        context.load_font(*settings::FONT_DETAILS);
    }

    fn on_exit(&mut self, context: &mut BarnContext) {}

    fn get_name(&mut self) -> String {
        String::from("credits")
    }
}
