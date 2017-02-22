use Pixel;
use cgmath::Vector3;

pub struct Tunel {
    pub size: Vector3<f64>,
}

impl Tunel {
    pub fn new(size: [f64; 3]) -> Tunel {
        Tunel {
            size: Vector3::from(size),
        }
    }
}
