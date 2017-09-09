use piston_window::{Input, PistonWindow};

pub enum Flow {
    StartGame,
    PlayAgain,
    LoseGame,
}

pub enum State {
    StartMenu,
    PlayAgainMenu,
    Playing,
}

pub trait EventHandler {
    type Input;
    fn handle_event(&mut self, Input, &mut PistonWindow, &mut Self::Input) -> Option<Flow>;
}
