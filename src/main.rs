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
mod ui;

use game::GameConfig;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    // Try to read config file, fallback to the default config otherwise
    let config = File::open("resources/config.json").ok().and_then(|mut f| {
        let mut s = String::new();
        f.read_to_string(&mut s).ok().and_then(|_| serde_json::from_str(&s).ok())
    }).unwrap_or(GameConfig::default());

    game::Game::new(config).run();
}
