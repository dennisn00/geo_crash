use graphics::Viewport;

enum GameState{
    Init,
    Preparing,
    Running{elapsed_time: f64, kills: i8},
    Death,
}

pub struct App{
    game_state: GameState,
    master: Master,
}

impl App{

    pub fn new() -> Self {
        App {
            game_state: GameState::Init,
        }
    }

    /*this returns a view object coreesponding to the current GameState
    TODO: implement view for all GameStates */
    pub fn view(&self) -> View {
        match self.game_state {
            GameState::Running {elapsed_time, kills} => View{

            }
            GameState::Init => {}
            GameState::Preparing => {}
            GameState::Death => {}
        }
    }
}

mod view;