extern crate boxcrash;
extern crate cgmath;

use boxcrash::game::{Game, GameConfig};
use boxcrash::Pixel;
use cgmath::Vector3;

fn main() {
    let config = GameConfig {
        title: "Box Crash",
        screen_size: Pixel::new(800, 600),
        ups: 60,
        max_fps: 60,
    };
    Game::new(config).run();
}
