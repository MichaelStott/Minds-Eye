extern crate sdl2;

mod camera;
mod context;
mod eye;

mod game_state;
mod input_handler;
mod physics;
mod player;
mod resource_manager;
mod settings;
mod start_menu_state;
mod state;
mod tile;


use crate::start_menu_state::StartMenuState;
use crate::game_state::GameState;
use crate::state::State;
use context::Context;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::video::FullscreenType;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    // Initialize SDL2 window.
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut window = video_subsystem
        .window("Mind's Eye", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    //window.set_fullscreen(FullscreenType::Desktop);

    // Initialize sound.
    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
    let _mixer_context =
        sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;

    // Set initial menu state.
    let mut state: Box<dyn State> = Box::new(StartMenuState { selected_option: 0});
    let mut events = sdl_context.event_pump()?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    

    // Initial context instantiation.
    let mut texture_creator = canvas.texture_creator();
    let mut font_context= sdl2::ttf::init().unwrap();
    let mut context = Context::new(&mut texture_creator, &mut font_context);

    // TODO: This should be handled in the game state....
    //context.load_level(String::from("res/levels/level1.txt"));
    context.camera.width = (canvas.output_size().unwrap().0) as i32;
    context.camera.height = (canvas.output_size().unwrap().1) as i32;

    state.on_enter(&mut context);
    // Main game loop.
    'running: loop {
        // Check if the game loop should be exited.
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // State handling logic.
        let new_state = context.update(&mut *state, &mut events);
        context.draw(&mut *state, &mut canvas);
        match new_state {
            Some(x) => {
                state.on_exit(&mut context);
                state = x;
                state.on_enter(&mut context);
            }
            None => {
                // No state change has occurred.
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
