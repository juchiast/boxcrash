use Color;
use color;
use Pixel;
use cgmath::{Vector3, Vector2};
use rand;
use camera::Camera;

pub struct Car {
    pub size: Vector3<f64>,
    pub position: Vector3<f64>,
    pub speed: f64,
    pub turn_speed: f64,
    pub color: Color,
}

pub struct CarRules {
    pub size: [(f64, f64); 3],
    pub position: [(f64, f64); 3],
    pub speed: (f64, f64),
    pub turn_speed: (f64, f64),
    pub color: Vec<Color>,
}

impl Car {
    pub fn new_random(rules: &CarRules) -> Car {
        Car {
            size: Vector3::new(rnd(rules.size[0]), rnd(rules.size[1]), rnd(rules.size[2])),
            position: Vector3::new(rnd(rules.size[0]), rnd(rules.size[1]), rnd(rules.size[2])),
            speed: rnd(rules.speed),
            turn_speed: rnd(rules.turn_speed),
            color: if rules.color.is_empty() {
                color::BLUE
            } else {
                rules.color[rand::random::<usize>() % rules.color.len()]
            }
        }
    }

    pub fn render(&self, camera: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        let mut front = [self.position; 4];
        front[0].y += self.size.y; front[1].y += self.size.y;
        front[0].x -= self.size.x/2.; front[3].x -= self.size.x/2.;
        front[1].x += self.size.x/2.; front[2].x += self.size.x/2.;
        let mut rear = front.clone();
        for i in 0..4 {
            rear[i].z += self.size.z;
        }
        let mut ret = Vec::new();
        for i in 0..4 {
            ret.push(camera.render_line(&front[i], &front[(i+1)%4]));
            ret.push(camera.render_line(&rear[i], &rear[(i+1)%4]));
            ret.push(camera.render_line(&front[i], &rear[i]));
        }
        ret.into_iter().filter(|x| x.is_some()).map(|x| (x.unwrap(), self.color)).collect()
    }
}

fn rnd((a, b): (f64, f64)) -> f64 {
    use std::f64;
    if a-b == 0. { a }
    else {
        let (a, b) = (f64::min(a, b), f64::max(a, b));
        rand::random::<f64>()%(b-a) + a
    }
}
