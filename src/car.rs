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
    pub jump_v: f64,
    pub jump_a: f64,
    pub jumping: bool,
    pub current_t: f64,
    pub jump_turn_decrease: f64,
}

pub struct CarRules {
    pub size: [(f64, f64); 3],
    pub position: [(f64, f64); 3],
    pub speed: (f64, f64),
    pub turn_speed: (f64, f64),
    pub color: Vec<Color>,
    pub jump_turn_decrease: f64,
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
            },
            jump_v: 0.,
            jump_a: 0.,
            jumping: false,
            current_t: 0.,
            jump_turn_decrease: rules.jump_turn_decrease,
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
        let speed = if self.jumping {
            self.turn_speed/self.jump_turn_decrease
        } else {
            self.turn_speed
        };
        match *turn {
            Turn::Left => self.position.x -= dt*speed,
            Turn::Right => self.position.x += dt*speed,
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
    pub fn start_jump(&mut self) {
        if self.jumping || self.position.y > 0. {}
        else {
            self.jumping = true;
            self.current_t = 0.;
        }
    }
    pub fn update_jump(&mut self, dt: f64) {
        if self.jumping {
            self.current_t += dt;
            self.position.y = self.current_t*(self.jump_v - 0.5*self.jump_a*self.current_t);
            if self.position.y < 0. {
                self.position.y = 0.;
                self.jumping = false;
            }
        }
    }
    pub fn hit(&self, bullet: &[Vector3<f64>; 3]) -> bool {
        let (x, y) = (bullet[0], bullet[0]+bullet[1]);
        let check = |x: &Vector3<f64>| {
            f64::abs(x.x-self.position.x) < self.size.x/2. &&
                x.y>=self.position.y && x.y-self.position.y < self.size.y &&
                x.z>=self.position.z && x.z-self.position.z < self.size.z
        };
        check(&x) || check(&y)
    }
}
