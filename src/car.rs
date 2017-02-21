use Color;
use color;
use Pixel;
use cgmath::Vector3;
use rand;

pub struct Car {
    pub position: Vector3<f64>,
    pub length: f64,
    pub speed: f64,
    pub turn_speed: f64,
    pub color: Color,
}

pub struct CarRules {
    pub position: Option<([f64; 3], [f64; 3])>,
    pub length: Option<(f64, f64)>,
    pub speed: Option<(f64, f64)>,
    pub turn_speed: Option<(f64, f64)>,
    pub color: Vec<Color>,
}

impl Car {
    pub fn new_random(rules: &CarRules) -> Car {
        Car {
            position: match rules.position {
                None => Vector3::from(rand::random::<[f64; 3]>()),
                Some((a, b)) => Vector3::from({
                    let mut s = [0f64; 3];
                    for i in 0..3 {
                        s[i] = rnd(a[i], b[i]);
                    }
                    s
                }),
            },
            length: match rules.length {
                None => rand::random(),
                Some((a, b)) => rnd(a, b),
            },
            speed: match rules.speed {
                None => rand::random(),
                Some((a, b)) => rnd(a, b),
            },
            turn_speed: match rules.turn_speed {
                None => rand::random(),
                Some((a, b)) => rnd(a, b),
            },
            color: if rules.color.is_empty() {
                color::BLUE
            } else {
                rules.color[rand::random::<usize>() % rules.color.len()]
            }
        }
    }
}

fn rnd(a: f64, b: f64) -> f64 {
    use std::f64;
    if a-b == 0. { a }
    else {
        let (a, b) = (f64::min(a, b), f64::max(a, b));
        rand::random::<f64>()%(b-a) + a
    }
}
