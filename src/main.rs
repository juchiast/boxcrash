extern crate rand;
extern crate cgmath;
extern crate piston_window;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate conrod;

mod color;
mod world;
mod tunel;
mod car;
mod camera;
mod bot;
mod game;
mod ui;

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

use game::GameConfig;
use std::f64::consts::PI;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let default_config = GameConfig {
        title: "Box Crash".to_owned(),
        screen_size: Pixel::new(800, 600),
        ups: 60,
        max_fps: 60,
        tunel_size: [15., 8., 150.],
        player_size: [1.5, 0.8, 3.],
        player_speed: (20., 120.),
        player_turn_speed: 15.,
        bot_size: [(1., 4.), (0.5, 2.5), (2.5, 8.)],
        bot_speed: (20., 120.),
        bot_turn_speed: (5., 20.),
        divider_size: [1., 7.],
        camera_height: 3.,
        camera_distance: 5.5,
        decor_distance: 8.,
        sprint_factor: 15.,
        spawn_time: (0.25, 1.),
        game_sprint: 1.,
        game_max_speed: 80.,
        player_jump_v: 7.,
        player_jump_a: 5.,
        jump_turn_decrease: 3.,
        jump_timeout: 8.,
        mouse_speed: PI/420.,
        trueshot_distance: 100.,
        bullet_stock: 15,
        recharge_time: 10.,
        bullet_len: 5.,
        bullet_speed: 100.,
        zoom_in: false,
    };

    // Try to read config file, fallback to the default config otherwise
    let mut config = File::open("resources/config.json").ok().and_then(|mut f| {
        let mut s = String::new();
        f.read_to_string(&mut s).ok().and_then(|_| serde_json::from_str(&s).ok())
    }).unwrap_or(default_config);

    ui::main(&mut config);
}
