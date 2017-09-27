use piston_window::{Event, PistonWindow};

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
    fn handle_event(&mut self, Event, &mut PistonWindow, &mut Self::Input) -> Option<Flow>;
}
