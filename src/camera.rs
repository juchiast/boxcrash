use std::f64;
use cgmath::prelude::*;
use cgmath::{Vector2, Vector3};
use Pixel;

const MAX_CAM_WIDTH: f64 = 1.0; 

#[derive(Clone)]
pub struct Camera {
    pub eye: Vector3<f64>,
    c: Vector3<f64>,
    axis_x: Vector3<f64>,
    axis_y: Vector3<f64>,
    zoom_factor: f64,
    screen_size: Pixel,
}

impl Camera {
    pub fn new(size: Pixel, location: Vector3<f64>) -> Camera {
        let c = Vector3::new(0., 0., 0.5);
        Camera {
            eye: location,
            c: c,
            axis_x: Vector3::new(1.0, 0.0, 0.0),
            axis_y: Vector3::new(0.0, 1.0, 0.0),
            zoom_factor: size.w as f64 / MAX_CAM_WIDTH,
            screen_size: size,
        }
    }

    pub fn render(&self, x: &Vector3<f64>) -> Option<Vector2<f64>> {
        let centre = self.eye + self.c;
        let side = |x: &Vector3<f64>| self.c.dot(x-centre);
        if side(&x)*side(&self.eye) > 0. {
            None
        } else {
            let x = (self.c.magnitude2()/self.c.dot(x-self.eye))*(x-self.eye) - self.c;
            Some(self.transform(&x))
        }
    }

    pub fn render_line(&self, x: &Vector3<f64>, y: &Vector3<f64>) -> Option<[Vector2<f64>; 2]> {
        let centre = self.eye + self.c;
        let (x1, y1) = (self.render(x), self.render(y));
        if x1.is_some() && y1.is_some() {
            Some([x1.unwrap(), y1.unwrap()])
        } else if x1.is_none() && y1.is_none() {
            None
        } else {
            let (a, v) = if x1.is_some() { (x, y-x) } else { (y, x-y) };
            let t = self.c.dot(centre-a) / self.c.dot(v);
            let x = t*v + a - centre;
            Some([
                 if x1.is_some(){x1.unwrap()} else {y1.unwrap()},
                 self.transform(&x)
            ])
        }
    }

    fn transform(&self, x: &Vector3<f64>) -> Vector2<f64> {
        if f64::abs(self.c.dot(x.clone())) > 1e-9 {
            panic!("Unexpected error in `fn Camera::transform`: {:?} {:?}, {}", self.c, x, self.c.dot(x.clone()));
        }
        let a = x.dot(self.axis_x)/self.axis_x.magnitude();
        let b = x.dot(self.axis_y)/self.axis_y.magnitude();
        let w = a*self.zoom_factor + self.screen_size.w as f64 / 2.;
        let h = self.screen_size.h as f64 - (b*self.zoom_factor+self.screen_size.h as f64 / 2.);
        Vector2::new(w, h)
    }

    pub fn test(&mut self) {
        use std::f64;
        let rotate_y = |v: Vector3<f64>, t: f64| -> Vector3<f64> {
            let sin = f64::sin(t);
            let cos = f64::cos(t);
            Vector3::new(v.x*cos + v.z*sin, v.y, -v.x*sin + v.z*cos)
        };
        let rotate_x = |v: Vector3<f64>, t: f64| -> Vector3<f64> {
            let sin = f64::sin(t);
            let cos = f64::cos(t);
            Vector3::new(v.x, v.y*cos - v.z*sin, v.y*sin + v.z*cos)
        };
        let x = f64::consts::PI/120.;
        self.c = rotate_y(self.c, x);
        self.axis_x = rotate_y(self.axis_x, x);
        self.axis_y = rotate_y(self.axis_y, x);
        self.c = rotate_x(self.c, x);
        self.axis_x = rotate_x(self.axis_x, x);
        self.axis_y = rotate_x(self.axis_y, x);
    }
}
