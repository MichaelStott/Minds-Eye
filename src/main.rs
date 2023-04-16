#![windows_subsystem = "windows"]
use barn::game::barn_context::BarnContext;
use barn::game::game::Game;
use barn::game::state::State;
use game::start_menu_state::StartMenuState;

mod game;
mod settings;
use crate::settings as config;

pub fn main() -> Result<(), String> {
    // Create game window.
    let mut game: Game = Game::new(&String::from(config::TITLE), 800, 600, false);

    // Create initial state and context.
    let state: Box<dyn State<BarnContext>> = Box::new(StartMenuState::new(0));
    let context = BarnContext::new(&mut game);

    // Start main game loop.
    game.run(context, state)
}
