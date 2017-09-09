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

use piston_window::*;
use std::io::prelude::*;
use std::fs::File;
use game::GameConfig;
use control::{Flow, EventHandler};

fn main() {
    // Try to read config file, fallback to the default config otherwise
    let config = File::open("resources/config.json").ok().and_then(|mut f| {
        let mut s = String::new();
        f.read_to_string(&mut s).ok().and_then(|_| serde_json::from_str(&s).ok())
    }).unwrap_or(GameConfig::default());

    let mut window: PistonWindow = WindowSettings::new(
        config.title.clone(), [config.screen_size.w, config.screen_size.h])
        .exit_on_esc(true)
        .build()
        .expect("Cannot create window.");
    window.set_ups(config.ups);
    window.set_max_fps(config.max_fps);
    window.set_capture_cursor(true);

    let mut game = game::Game::new(config, &window);

    while let Some(event) = window.next() {
        let flow = game.handle_event(event, &mut window);

        if let Some(flow) = flow {
            use Flow::*;
            match flow {
                LoseGame => break,
            }
        }
    }
}
