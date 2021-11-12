extern crate sdl2;
mod barn;

mod camera;
mod credits_state;
mod eye;
mod fire;
mod game_state;
mod help_state;
mod level_select_state;
mod physics;
mod player;
mod resource_manager;
mod settings;
mod start_menu_state;
mod texture_manager;
mod tile;

use std::path::Path;
use crate::camera::Camera;
use crate::barn::game::context::Context;
use crate::barn::game::state::State;
use crate::start_menu_state::StartMenuState;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::video::FullscreenType;
use std::process;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    // Initialize SDL2 window.
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut window = video_subsystem
        .window("Mind's Eye", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    //window.set_fullscreen(FullscreenType::True).unwrap();

    // Initialize sound.
    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
    let _mixer_context =
        sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).unwrap();
    sdl2::mixer::allocate_channels(6);

    // Set initial menu state.
    let mut state: Box<dyn State> = Box::new(StartMenuState { 
        selected_option: 0,
        tiles: Vec::new(),
        blocks: Vec::new(),
        eyes: Vec::new(),
        move_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/push.ogg")).unwrap(),
        select_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/select.ogg")).unwrap(),
        camera: Camera::new(),
        //socket_tex: texture_creator.load_texture(Path::new("res/img/socket.png")).unwrap(),
        enter_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/enter.ogg")).unwrap(),
    });
    let mut events = sdl_context.event_pump().unwrap();
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    canvas.set_logical_size(800, 600).unwrap();

    // Initial context instantiation.
    let mut texture_creator = canvas.texture_creator();
    let mut font_context = sdl2::ttf::init().unwrap();
    let mut context = Context::new(&mut texture_creator, &mut font_context);
    //context.music.play(-1).unwrap();
    //context.move_fx.set_volume(50);

    env_logger::init();
    
    //context.camera.width = (canvas.output_size().unwrap().0) as i32;
    //context.camera.height = (canvas.output_size().unwrap().1) as i32;

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
                log::debug!("Switched to state: {}", state.get_name());
            }
            None => {
                // No state change has occurred.
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
