use piston_window::{Input, PistonWindow};

pub enum Flow {
    LoseGame,
}

pub trait EventHandler {
    fn handle_event(&mut self, e: Input, window: &mut PistonWindow) -> Option<Flow>;
}