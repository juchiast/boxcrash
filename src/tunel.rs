use cgmath::{Vector2, Vector3};
use color::*;
use camera::Camera;

pub struct Tunel {
    pub size: Vector3<f64>,
    pub color: Color,
}

impl Tunel {
    pub fn new(size: [f64; 3]) -> Tunel {
        Tunel {
            size: size.into(),
            color: BLUE,
        }
    }

    pub fn render(&self, camera: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        vec![
            ((0., 0., 0.), (0., 0., self.size.z)),
            ((self.size.x, 0., 0.), (self.size.x, 0., self.size.z)),
            ((0., self.size.y, 0.), (0., self.size.y, self.size.z)),
            ((self.size.x, self.size.y, 0.), (self.size.x, self.size.y, self.size.z)),
        ].into_iter().map(|(a, b)| camera.render_line(&a.into(), &b.into()))
         .filter_map(|x| x.map(|x| (x, self.color))).collect()
    }
}
