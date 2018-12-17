use super::camera::Camera;
use crate::color::*;
use cgmath::{Vector2, Vector3};

// Present a car that can be drawed, check for collision
// with other car and bullet, turn left/right, move forward
// and jump.
pub trait Car {
    fn render(&self, _: &Camera) -> Vec<([Vector2<f64>; 2], Color)>;
    fn crashed(&self, _: &Self) -> bool;
    fn hit(&self, _: &[Vector3<f64>; 3]) -> bool;
    fn forward(&mut self, dt: f64, outside_speed: f64);
    fn turn_left(&mut self, dt: f64);
    fn turn_right(&mut self, dt: f64);
    fn update_jump(&mut self, dt: f64);
    fn jump(&mut self);
    fn pos(&self) -> Vector3<f64>;
    fn turn_speed(&self) -> f64;
}

// Car with shape of a box
#[derive(Clone)]
pub struct BoxCar {
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

impl Car for BoxCar {
    fn render(&self, camera: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        let mut front = [self.position; 4];
        front[0].y += self.size.y;
        front[1].y += self.size.y;
        front[0].x -= self.size.x / 2.;
        front[3].x -= self.size.x / 2.;
        front[1].x += self.size.x / 2.;
        front[2].x += self.size.x / 2.;
        let mut rear = front;
        for x in &mut rear {
            x.z += self.size.z;
        }
        let mut ret = Vec::new();
        for i in 0..4 {
            ret.push(camera.render_line(&front[i], &front[(i + 1) % 4]));
            ret.push(camera.render_line(&rear[i], &rear[(i + 1) % 4]));
            ret.push(camera.render_line(&front[i], &rear[i]));
        }
        ret.into_iter()
            .filter_map(|x| x.map(|x| (x, self.color)))
            .collect()
    }

    fn turn_left(&mut self, dt: f64) {
        self.position.x -= dt * self.turn_speed();
    }
    fn turn_right(&mut self, dt: f64) {
        self.position.x += dt * self.turn_speed();
    }
    fn pos(&self) -> Vector3<f64> {
        self.position
    }
    fn crashed(&self, a: &Self) -> bool {
        (f64::abs(self.position.x - a.position.x) < (self.size.x + a.size.x) / 2.)
            && ((self.position.z < a.position.z && a.position.z - self.position.z < self.size.z)
                || (self.position.z >= a.position.z && self.position.z - a.position.z < a.size.z))
            && ((self.position.y < a.position.y && a.position.y - self.position.y < self.size.y)
                || (self.position.y >= a.position.y && self.position.y - a.position.y < a.size.y))
    }
    fn jump(&mut self) {
        if self.jumping || self.position.y > 0. {
        } else {
            self.jumping = true;
            self.current_t = 0.;
        }
    }
    fn forward(&mut self, dt: f64, outside_speed: f64) {
        self.position.z -= dt * (self.speed + outside_speed);
        self.update_jump(dt);
    }
    fn update_jump(&mut self, dt: f64) {
        if self.jumping {
            self.current_t += dt;
            self.position.y = self.current_t * (self.jump_v - 0.5 * self.jump_a * self.current_t);
            if self.position.y < 0. {
                self.position.y = 0.;
                self.jumping = false;
            }
        }
    }
    fn hit(&self, bullet: &[Vector3<f64>; 3]) -> bool {
        let (x, y) = (bullet[0], bullet[0] + bullet[1]);
        let check = |x: &Vector3<f64>| {
            f64::abs(x.x - self.position.x) < self.size.x / 2.
                && x.y >= self.position.y
                && x.y - self.position.y < self.size.y
                && x.z >= self.position.z
                && x.z - self.position.z < self.size.z
        };
        check(&x) || check(&y)
    }
    fn turn_speed(&self) -> f64 {
        if self.jumping {
            self.turn_speed / self.jump_turn_decrease
        } else {
            self.turn_speed
        }
    }
}
