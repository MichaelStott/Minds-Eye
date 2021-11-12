use crate::barn::game::state::State;
use crate::barn::input::keyboard_handler::KeyboardHandler;
use crate::resource_manager::FontDetails;
use crate::resource_manager::ResourceManager;

use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use sdl2::EventPump;

type TextureManager<'l, T> = ResourceManager<'l, String, Texture, TextureCreator<T>>;
type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

pub struct Context<'a> {
    pub texture_manager: TextureManager<'a, WindowContext>,
    pub font_manager: FontManager<'a>,
    pub font_details: FontDetails,
    pub input: KeyboardHandler,
}

impl<'a> Context<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        ttf_context: &'a Sdl2TtfContext,
    ) -> Self {
        Context {
            input: KeyboardHandler::new(),
            texture_manager: TextureManager::new(&texture_creator),
            font_manager: FontManager::new(ttf_context),
            font_details: FontDetails {
                path: String::from("res/fonts/VeniceClassic.ttf"),
                size: 19,
            },
        }
    }

    pub fn get_input_handler(&mut self) -> &mut KeyboardHandler {
        &mut self.input
    }

    pub fn update(&mut self, state: &mut dyn State, event: &mut EventPump) -> Option<Box<dyn State>>
    where
        Self: std::marker::Sized,
    {
        self.get_input_handler().update(event);
        state.update(self)
    }

    pub fn draw(&mut self, state: &mut dyn State, canvas: &mut WindowCanvas)
    where
        Self: std::marker::Sized,
    {
        state.draw(self, canvas);
        self.get_input_handler().refresh_prev();
    }
}
