use Color;
use Pixel;
use cgmath::Vector3;

pub struct Car {
    front: [Vector3<f64>; 4],
    rear: [Vector3<f64>; 4],
    speed: f64,
    turn_speed: f64,
    color: Color,
}
