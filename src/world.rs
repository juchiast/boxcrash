use tunel::Tunel;
use car::{ Car, CarRules };
use color;
use cgmath::Vector3;

pub struct World {
    pub tunel: Tunel,
    pub player: Car,
    pub bots: Vec<Car>,
    pub rules: CarRules,
}

impl World {
    pub fn new() -> World {
        let tunel = Tunel::new();
        let player = Car {
            size: Vector3::new(1.5, 2., 3.),
            position: Vector3::new(tunel.size.x/2., 0., 3.),
            speed: 8.,
            turn_speed: 3.,
            color: color::YELLOW,       
        };
        let rules = CarRules {
            size: [(1., 2.), (1.5, 2.), (3., 4.)],
            position: [(0., tunel.size.x), (0., 0.), (0., tunel.length)],
            speed: (5., 10.),
            turn_speed: (0., 0.),
            color: Vec::new(),
        };
        World {
            tunel: tunel,
            player: player,
            rules: rules,
            bots: Vec::new(),
        }
    }
}
