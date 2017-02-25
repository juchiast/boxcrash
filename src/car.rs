use Color;
use color::*;
use cgmath::{Vector3, Vector2};
use camera::Camera;
use game::Turn;
use rnd;
use rand;

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
            position: Vector3::new(rnd(rules.position[0]), rnd(rules.position[1]), rnd(rules.position[2])),
            speed: rnd(rules.speed),
            turn_speed: rnd(rules.turn_speed),
            color: if rules.color.is_empty() {
                RED
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

    pub fn turn(&mut self, turn: &Turn, dt: f64) {
        match *turn {
            Turn::Left => self.position.x -= dt*self.turn_speed,
            Turn::Right => self.position.x += dt*self.turn_speed,
            Turn::None => (),
        }
    }
    pub fn rear_z(&self) -> f64 {
        self.position.z + self.size.z
    }
    pub fn crash(&self, a: &Car) -> bool {
        (f64::abs(self.position.x - a.position.x) < (self.size.x+a.size.x)/2.) &&
            ((self.position.z<a.position.z && a.position.z-self.position.z < self.size.z) ||
             (self.position.z>=a.position.z && self.position.z-a.position.z < a.size.z)) &&
            ((self.position.y<a.position.y && a.position.y-self.position.y < self.size.y) ||
             (self.position.y>=a.position.y && self.position.y-a.position.y < a.size.y))
    }
}
