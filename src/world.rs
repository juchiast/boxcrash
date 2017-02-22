use tunel::Tunel;
use car::Car;
use Color;
use color::*;
use cgmath::{Vector2, Vector3};
use game::GameConfig;
use camera::Camera;

pub struct World {
    pub tunel: Tunel,
    pub player: Car,
    pub bots: Vec<Car>,
}

impl World {
    pub fn new(config: &GameConfig) -> World {
        let player = Car {
            size: Vector3::from(config.player_size),
            position: Vector3::new(config.tunel_size[0]/2., 0., 6.),
            speed: config.player_speed,
            turn_speed: config.player_turn_speed,
            color: YELLOW,
        };

        World {
            tunel: Tunel::new(config.tunel_size),
            player: player,
            bots: Vec::new(),
        }
    }

    pub fn render(&self, camera: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        let mut ret = Vec::new();
        ret.append(&mut self.tunel.render(camera));
        ret.append(&mut self.player.render(camera));
        for bot in &self.bots {
            ret.append(&mut bot.render(camera));
        }
        ret
    }
}
