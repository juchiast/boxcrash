extern crate rand;
extern crate cgmath;
extern crate piston_window;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod world;
mod tunel;
mod car;
mod camera;
mod bot;
pub mod game;

#[derive(Clone, Serialize, Deserialize)]
pub struct Pixel {
    pub w: u32,
    pub h: u32,
}
impl Pixel {
    pub fn new(w: u32, h: u32) -> Pixel { Pixel {w: w, h: h} }
}

mod color {
    pub type Color = [f32; 4];
    pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
    pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
    pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
    pub const ORANGE: Color = [1.0, 0.5, 0.0, 1.0];
    pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
    pub const VIOLET: Color = [0.6, 0.0, 1.0, 1.0];
    pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
    pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
    pub const PALE: Color = [0.3, 0.3, 0.3, 0.1];
    pub fn pale(mut c: Color, f: f32) -> Color {
        c[3] = f;
        c
    }
}

fn rnd((a, b): (f64, f64)) -> f64 {
    let (a, b) = (f64::min(a, b), f64::max(a, b));
    rand::random::<f64>()*(b-a) + a
}

use game::{Game, GameConfig};
use std::f64::consts::PI;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let default_config = GameConfig {
        title: "Box Crash".to_owned(),
        fullscreen: true,
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
    let config = File::open("resources/config.json").ok().and_then(|mut f| {
        let mut s = String::new();
        f.read_to_string(&mut s).ok().and_then(|_| serde_json::from_str(&s).ok())
    }).unwrap_or(default_config);
    Game::new(config).run();
}
