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
            size: Vector3::new(1.5, 2., 3.),
            position: Vector3::new(config.tunel_size[0]/2., 0., 3.),
            speed: 8.,
            turn_speed: 3.,
            color: color::YELLOW,       
        };

        World {
            tunel: Tunel::new(config.tunel_size),
            player: player,
            bots: Vec::new(),
        }
    }
}
