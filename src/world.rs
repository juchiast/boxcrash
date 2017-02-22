use tunel::Tunel;
use car::{ Car, CarRules };
use color;
use cgmath::Vector3;
use game::GameConfig;

pub struct World {
    pub tunel: Tunel,
    pub player: Car,
    pub bots: Vec<Car>,
}

impl World {
    pub fn new(config: &GameConfig) -> World {
        let player = Car {
            size: Vector3::from(config.player_size),
            position: Vector3::new(config.tunel_size[0]/2., 0., 3.),
            speed: config.player_speed,
            turn_speed: config.player_turn_speed,
            color: color::YELLOW,       
        };

        World {
            tunel: Tunel::new(config.tunel_size),
            player: player,
            bots: Vec::new(),
        }
    }
}
