use barn::game::game::Game;
use barn::game::context::Context;
use barn::game::state::State;

mod game;
mod settings;
use crate::settings as config;
use crate::game::start_menu_state::StartMenuState;

pub fn main() -> Result<(), String> {
    // Create game window.
    let mut game: Game = Game::new(&String::from(config::TITLE), 800, 600, false);

    // Create initial state and context.
    let state: Box<dyn State> = Box::new(StartMenuState::new(0));
    let context = Context::new(&game);

    // Start main game loop.
    game.run(context, state)
}
