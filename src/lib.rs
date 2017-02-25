extern crate rand;
extern crate cgmath;
extern crate piston_window;

fn rnd((a, b): (f64, f64)) -> f64 {
    let (a, b) = (f64::min(a, b), f64::max(a, b));
    rand::random::<f64>()*(b-a) + a
}

#[derive(Clone)]
pub struct Pixel {
    pub w: u32,
    pub h: u32,
}
impl Pixel {
    pub fn new(w: u32, h: u32) -> Pixel { Pixel {w: w, h: h} }
}

type Color = [f32; 4];
mod color {
    use Color;
    pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
    pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
    pub const ORANGE: Color = [1.0, 0.5, 0.0, 1.0];
    pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
    pub const VIOLET: Color = [0.6, 0.0, 1.0, 1.0];
    pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
}

mod world;
mod tunel;
mod car;
mod camera;
pub mod game;
