use Color;
use color;
use Pixel;
use cgmath::Vector3;
use rand;

pub struct Car {
    pub size: Vector3<f64>,
    pub position: Vector3<f64>,
    pub speed: f64,
    pub turn_speed: f64,
    pub color: Color,
}

pub struct CarRules {
    pub size: [(f64, f64); 3],
    pub position: [(f64, f64); 3],
    pub speed: (f64, f64),
    pub turn_speed: (f64, f64),
    pub color: Vec<Color>,
}

impl Car {
    pub fn new_random(rules: &CarRules) -> Car {
        Car {
            size: Vector3::new(rnd(rules.size[0]), rnd(rules.size[1]), rnd(rules.size[2])),
            position: Vector3::new(rnd(rules.size[0]), rnd(rules.size[1]), rnd(rules.size[2])),
            speed: rnd(rules.speed),
            turn_speed: rnd(rules.turn_speed),
            color: if rules.color.is_empty() {
                color::BLUE
            } else {
                rules.color[rand::random::<usize>() % rules.color.len()]
            }
        }
    }
}

fn rnd((a, b): (f64, f64)) -> f64 {
    use std::f64;
    if a-b == 0. { a }
    else {
        let (a, b) = (f64::min(a, b), f64::max(a, b));
        rand::random::<f64>()%(b-a) + a
    }
}
