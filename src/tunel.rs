use Pixel;
use cgmath::{Vector2, Vector3};
use Color;
use color::*;
use camera::Camera;

pub struct Tunel {
    pub size: Vector3<f64>,
    pub color: Color,
}

impl Tunel {
    pub fn new(size: [f64; 3]) -> Tunel {
        Tunel {
            size: Vector3::from(size),
            color: BLUE,
        }
    }

    pub fn render(&self, camera: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        vec![
            camera.render_line(&Vector3::new(0., 0., 0.), &Vector3::new(0., 0., self.size.z)),
            camera.render_line(&Vector3::new(self.size.x, 0., 0.), &Vector3::new(self.size.x, 0., self.size.z)),
            camera.render_line(&Vector3::new(0., self.size.y, 0.), &Vector3::new(0., self.size.y, self.size.z)),
            camera.render_line(&Vector3::new(self.size.x, self.size.y, 0.), &Vector3::new(self.size.x, self.size.y, self.size.z)),
        ].into_iter().filter(|x| x.is_some()).map(|x| (x.unwrap(), self.color)).collect()
    }
}
