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
            position: Vector3::new(tunel.size.x/2., 0., 3.),
            length: 8.,
            speed: 8.,
            turn_speed: 3.,
            color: color::YELLOW,       
        };
        let rules = CarRules {
            position: Some(([0., 0., tunel.length], [tunel.size.x, 0., tunel.length])),
            length: Some((5., 10.)),
            speed: Some((5., 10.)),
            turn_speed: Some((1., 5.)),
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
