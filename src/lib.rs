extern crate piston_window;

use crate::master::Master;
use crate::view::View;

mod player;
mod master;
mod view;
mod game_object;

enum GameState{
    Init,
    Preparing,
    Running{elapsed_time: f64, kills: i8},
    Death,
}

pub struct App{
    game_state: GameState,
    pub master: Master,
}

impl App{

    pub fn new() -> Self {
        App {
            game_state: GameState::Init,
            master: Master::new(),
        }
    }

    /*this returns a View object coresponding to the current GameState
    TODO: implement View for all GameStates */
    pub fn view(&self) -> View {
        match self.game_state {
            GameState::Running {elapsed_time, kills} =>
                View::new(),

            //TODO: return Views for other Gamestats
            GameState::Init => {View::new()},
            GameState::Preparing => {View::new()},
            GameState::Death => {View::new()},
        }
    }
}

