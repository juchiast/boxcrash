extern crate rand;
extern crate cgmath;
extern crate piston_window;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
#[macro_use] extern crate conrod;

type Rendered = Vec<([cgmath::Vector2<f64>; 2], color::Color)>;

// Pixel present a point in the window and window's size
#[derive(Clone, Serialize, Deserialize)]
pub struct Pixel {
    pub w: u32,
    pub h: u32,
}
impl Pixel {
    pub fn new(w: u32, h: u32) -> Pixel { Pixel {w: w, h: h} }
}

// Return a random number between a and b
fn rnd((a, b): (f64, f64)) -> f64 {
    let (a, b) = (f64::min(a, b), f64::max(a, b));
    // `random::<f64>()` return a number between 0 and 1
    rand::random::<f64>()*(b-a) + a
}

mod color;
mod world;
mod tunel;
mod car;
mod camera;
mod bot;
mod game;
mod control;
mod conrod_helper;
mod menu;

use piston_window::*;
use std::io::prelude::*;
use std::fs::File;
use game::GameConfig;
use control::{Flow, EventHandler, State};
use conrod_helper::ConrodUI;
use menu::*;

fn main() {
    // Try to read config file, fallback to the default config otherwise
    let config = File::open("resources/config.json").ok().and_then(|mut f| {
        let mut s = String::new();
        f.read_to_string(&mut s).ok().and_then(|_| serde_json::from_str(&s).ok())
    }).unwrap_or(GameConfig::default());

    let size = config.screen_size.clone();

    let mut window: PistonWindow = WindowSettings::new(
        config.title.clone(), [size.w, size.h])
        .exit_on_esc(true)
        .build()
        .expect("Cannot create window.");
    window.set_ups(config.ups);
    window.set_max_fps(config.max_fps);
    window.set_capture_cursor(true);

    let mut game = game::Game::new(config.clone(), &window);
    let mut start_menu: ConrodUI<StartMenu> = ConrodUI::new(size.clone(), &mut window);
    let mut play_again_menu: ConrodUI<PlayAgainMenu> = ConrodUI::new(size.clone(), &mut window);

    let mut state = State::StartMenu;

    while let Some(event) = window.next() {
        let flow = match state {
            State::StartMenu => start_menu.handle_event(event, &mut window),
            State::Playing => game.handle_event(event, &mut window),
            State::PlayAgainMenu => play_again_menu.handle_event(event, &mut window),
        };

        if let Some(flow) = flow {
            use Flow::*;
            match flow {
                StartGame => state = State::Playing,
                LoseGame => state = State::PlayAgainMenu,
                PlayAgain => {
                    game = game::Game::new(config.clone(), &window);
                    state = State::Playing;
                },
            }
        }
    }
}
