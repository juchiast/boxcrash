extern crate boxcrash;
extern crate cgmath;

use boxcrash::game::{Game, GameConfig};
use boxcrash::Pixel;
use cgmath::Vector3;

fn main() {
    let config = GameConfig {
        screen_size: Pixel::new(800, 600),
    };
    Game::new(config).run();
}
