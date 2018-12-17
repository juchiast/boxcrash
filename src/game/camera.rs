use cgmath::prelude::*;
use cgmath::{Vector2, Vector3, vec3};

const MAX_CAM_WIDTH: f64 = 1.0;

// A struct to render points and lines in 3D plane to the screen
#[derive(Clone)]
pub struct Camera {
    // Location of the camera
    pub eye: Vector3<f64>,
    // Direction
    pub c: Vector3<f64>,
    axis_x: Vector3<f64>,
    axis_y: Vector3<f64>,
    zoom_factor: f64,
    screen_size: crate::Pixel,
}

impl Camera {
    pub fn new(size: crate::Pixel, location: Vector3<f64>) -> Camera {
        Camera {
            eye: location,
            c: vec3(0., 0., 0.5),
            axis_x: vec3(1.0, 0.0, 0.0),
            axis_y: vec3(0.0, 1.0, 0.0),
            zoom_factor: f64::from(size.w) / MAX_CAM_WIDTH,
            screen_size: size,
        }
    }

    // Render a single point, return None if we can't see it
    pub fn render(&self, x: &Vector3<f64>) -> Option<Vector2<f64>> {
        let centre = self.eye + self.c;
        let side = |x: &Vector3<f64>| self.c.dot(x - centre);
        if side(x) * side(&self.eye) > 0. {
            None
        } else {
            let x = (self.c.magnitude2() / self.c.dot(x - self.eye)) * (x - self.eye) - self.c;
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
            let (a, v) = if x1.is_some() { (x, y - x) } else { (y, x - y) };
            let t = self.c.dot(centre - a) / self.c.dot(v);
            let x = t * v + a - centre;
            Some([x1.unwrap_or_else(|| y1.unwrap()), self.transform(&x)])
        }
    }

    fn transform(&self, x: &Vector3<f64>) -> Vector2<f64> {
        if f64::abs(self.c.dot(*x)) > 1e-9 {
            panic!(
                "Unexpected error in `fn Camera::transform`: {:?} {:?}, {}",
                self.c,
                x,
                self.c.dot(x.clone())
            );
        }
        let a = x.dot(self.axis_x) / self.axis_x.magnitude();
        let b = x.dot(self.axis_y) / self.axis_y.magnitude();
        let w = a * self.zoom_factor + f64::from(self.screen_size.w) / 2.;
        let h = f64::from(self.screen_size.h)
            - (b * self.zoom_factor + f64::from(self.screen_size.h) / 2.);
        Vector2::new(w, h)
    }

    // Rotate the camera's direction around a centre and
    // the Ox, Oy axis (3D plane axis).
    // `x`, `y` is degree measured in Radian.
    pub fn rotate(&mut self, y: f64, x: f64, centre: Vector3<f64>) {
        use cgmath::{Basis3, Rad, Rotation3};
        let mut vec = self.eye - centre;
        let rotate_x = Basis3::from_angle_x(Rad(x));
        let rotate_y = Basis3::from_angle_y(Rad(y));
        for v in vec![&mut self.c, &mut self.axis_x, &mut self.axis_y, &mut vec] {
            *v = rotate_x.rotate_vector(*v);
            *v = rotate_y.rotate_vector(*v);
        }
        self.eye = centre + vec;
    }

    pub fn zoom_in(&mut self) {
        self.c *= 2.;
    }
}
