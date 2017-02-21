extern crate cgmath;

pub struct Pixel {
    pub w: u32,
    pub h: u32,
}
impl Pixel {
    fn new(w: u32, h: u32) -> Pixel { Pixel {w: w, h: h} }
}

type Color = [f64; 4];

mod world;
mod tunel;
mod car;
mod camera;
