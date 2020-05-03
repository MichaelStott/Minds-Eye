use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use std::collections::HashSet;

pub struct InputHandler {
    new_keys: HashSet<Keycode>,
    prev_keys: HashSet<Keycode>,
    current_keys: HashSet<Keycode>,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            new_keys: HashSet::new(),
            prev_keys: HashSet::new(),
            current_keys: HashSet::new() 
        }
    }

    pub fn update(&mut self, events: &mut EventPump) {
        let keys: HashSet<Keycode> = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        self.new_keys = &keys - &self.prev_keys;
        self.prev_keys = &self.prev_keys - &keys;
        self.current_keys = keys;
    }

    pub fn refresh_prev(&mut self) {
        self.prev_keys = self.current_keys.clone();
    }

    pub fn key_pressed(&mut self, key: &Keycode) -> bool {
        self.current_keys.contains(key)
    }

    pub fn key_just_pressed(&mut self, key: &Keycode) -> bool {
        self.new_keys.contains(key) && !self.prev_keys.contains(key)
    }

    pub fn key_released(&mut self, key: &Keycode) -> bool {
        !self.new_keys.contains(key) && self.prev_keys.contains(key)
    }
}