use std::f64;
use cgmath::prelude::*;
use cgmath::{Vector2, Vector3};
use Pixel;

const MAX_CAM_WIDTH: f64 = 1.0; 

pub struct Camera {
    eye: Vector3<f64>,
    c: Vector3<f64>,
    axis_x: Vector3<f64>,
    screen_size: Pixel,
    limit: Vector2<f64>,
    zoom_factor: f64,
}

impl Camera {
    pub fn new(size: Pixel, location: Vector2<f64>) -> Camera {
        let (w, h) = (size.w as f64, size.h as f64);
        Camera {
            eye: location.extend(0.0),
            c: Vector3::new(0.0, 0.0, 1.0),
            axis_x: Vector3::new(1.0, 0.0, 0.0),
            screen_size: size,
            limit: Vector2::new(MAX_CAM_WIDTH/2.0, MAX_CAM_WIDTH*h/w/2.0),
            zoom_factor: w / MAX_CAM_WIDTH,
        }
    }
    pub fn render(&self, x: Vector3<f64>) -> Option<Pixel> {
        let x = (self.c.magnitude()/self.c.dot(x-self.eye))*x- self.c;
        let a = x.dot(self.axis_x)/self.axis_x.magnitude();
        let b = f64::sqrt(x.magnitude2()-a*a);
        if f64::abs(a) > self.limit.x || f64::abs(b) > self.limit.y {
            None
        } else {
            let w = ((a+self.limit.x)*self.zoom_factor) as u32;
            let h = self.screen_size.h - ((b+self.limit.y)*self.zoom_factor) as u32;
            Some(Pixel::new(w, h))
        }
    }
}
