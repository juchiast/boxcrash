use Pixel;
use cgmath::Vector2;

pub struct Tunel {
    pub length: f64,
    pub size: Vector2<f64>,
}

impl Tunel {
    pub fn new() -> Tunel {
        Tunel {
            length: 150.0,
            size: Vector2::new(30.0, 20.0),
        }
    }
}
