extern crate boxcrash;

use boxcrash::game::{Game, GameConfig};
use boxcrash::Pixel;

fn main() {
    let config = GameConfig {
        title: "Box Crash",
        screen_size: Pixel::new(800, 600),
        ups: 60,
        max_fps: 60,
        tunel_size: [15., 8., 100.],
        player_size: [1.5, 0.8, 3.],
        player_speed: 30.,
        player_turn_speed: 10.,
        bot_size: [(1., 2.), (1.5, 2.), (3., 4.)],
        bot_speed: (30., 90.),
        bot_turn_speed: (0., 0.),
        divider_size: [1., 7.],
        camera_height: 3.,
        camera_distance: 5.5,
        decor_distance: 8.,
        sprint_factor: 2.,
        spawn_time: (0.25, 1.5),
    };
    Game::new(config).run();
}
