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
    fn handle_event(&mut self, Input, &mut PistonWindow) -> Option<Flow>;
}
