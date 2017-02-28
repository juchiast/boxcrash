use tunel::Tunel;
use car::*;
use color::*;
use cgmath::{Vector2, Vector3, vec3};
use cgmath::prelude::*;
use game::GameConfig;
use camera::Camera;
use std::collections::VecDeque;
use bot::{Bot, BoxRules};

pub struct World {
    pub tunel: Tunel,
    pub player: BoxCar,
    pub bots: VecDeque<Bot>,
    pub divider: Vector2<f64>,
    pub decor_distance: f64,
    pub divider_state: f64,
    pub decor_state: f64,
    pub bullets: Vec<[Vector3<f64>; 3]>,
}

impl World {
    fn divider_render(&self, camera: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        let color = self.tunel.color;
        let mut points = [vec3(self.tunel.size.x/2., 0., self.divider_state); 4];
        points[2].z -= self.divider.y; points[3].z -= self.divider.y;
        points[0].x -= self.divider.x/2.; points[3].x -= self.divider.x/2.;
        points[1].x += self.divider.x/2.; points[2].x += self.divider.x/2.;
        
        let mut ret = Vec::new();
        {
            let mut r = |a: &Vector3<f64>, b: &Vector3<f64>| {
                if let Some(rendered) = camera.render_line(a, b) {
                    ret.push((rendered, color));
                }
            };
            while points[0].z <= self.tunel.size.z {
                for i in 0..4 {
                    r(&points[i], &points[(i+1)%4]);
                }
                for i in 0..4 {
                    points[i].z += 2.*self.divider.y;
                }
            }
            points[0].z = self.tunel.size.z;
            points[1].z = self.tunel.size.z;
            r(&points[0], &points[3]);
            r(&points[1], &points[2]);
            r(&points[2], &points[3]);
        }
        ret
    }
    fn decor_render(&self, camera: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        let mut data = [
            vec3(0., 0., self.decor_state),
            vec3(0., self.tunel.size.y, self.decor_state),
            vec3(self.tunel.size.x, self.tunel.size.y, self.decor_state),
            vec3(self.tunel.size.x, 0., self.decor_state),
        ];
        let mut ret = Vec::new();
        while data[0].z <= self.tunel.size.z {
            for i in 0..3 {
                if let Some(rendered) = camera.render_line(&data[i], &data[i+1]) {
                    ret.push((rendered, self.tunel.color));
                }
            }
            for i in 0..4 {
                data[i].z += self.decor_distance;
            }
        }
        ret
    }

    pub fn new(config: &GameConfig) -> World {
        let player = BoxCar {
            size: config.player_size.into(),
            position: vec3(config.tunel_size[0]/2., 0., 10.),
            speed: config.player_speed.0,
            turn_speed: config.player_turn_speed,
            color: YELLOW,
            jump_v: config.player_jump_v,
            jump_a: config.player_jump_a,
            jumping: false,
            current_t: 0.,
            jump_turn_decrease: config.jump_turn_decrease,
        };

        World {
            tunel: Tunel::new(config.tunel_size),
            player: player,
            bots: VecDeque::new(),
            divider: config.divider_size.into(),
            divider_state: config.divider_size[1],
            decor_distance: config.decor_distance,
            decor_state: config.decor_distance,
            bullets: Vec::new(),
        }
    }

    pub fn render(&self, camera: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        let mut ret = Vec::new();
        ret.append(&mut self.tunel.render(camera));
        ret.append(&mut self.divider_render(camera));
        ret.append(&mut self.decor_render(camera));
        ret.append(&mut self.player.render(camera));
        for bot in &self.bots {
            ret.append(&mut bot.render(camera));
        }
        for x in &self.bullets {
            if let Some(rendered) = camera.render_line(&x[0], &(x[0]+x[1])) {
                ret.push((rendered, self.player.color));
            }
        }
        ret
    }
    pub fn update(&mut self, dt: f64, game_speed: f64) {
        self.player.update_jump(dt);
        let speed = game_speed + self.player.speed;
        self.divider_state -= dt*speed;
        if self.divider_state < 0. {
            self.divider_state += 2.*self.divider.y;
        }
        self.decor_state -= dt*speed;
        if self.decor_state < 0. {
            self.decor_state += self.decor_distance;
        }
        for ref mut x in &mut self.bots {
            x.drive(dt);
        }
        for i in 0..self.bots.len() {
            self.bots[i].forward(dt, speed);
            if i>0 {
                let mut j = i-1;
                while j>0 {
                    if self.bots[j].pos().z > self.bots[i].pos().z {
                        self.bots.swap(i, j);
                        j -= 1;
                    } else { break }
                }
            }
        }
        for ref mut x in &mut self.bullets {
            x[0] += dt*x[2];
        }
    }
    pub fn validate(&mut self) {
        let size = self.tunel.size;
        let car = |car: &mut BoxCar| {
            if car.position.x + car.size.x/2. > size.x {
                car.position.x = size.x - car.size.x/2.;
            } else if car.position.x - car.size.x/2. < 0. {
                car.position.x = car.size.x/2.;
            }
            if car.position.y + car.size.y > size.y {
                car.position.y = size.y - car.size.y;
            }
        };
        car(&mut self.player);
        if !self.bots.is_empty() {
            for ref mut x in &mut self.bots {
                car(&mut x.car);
            }
            let mut len = self.bots.len();
            let mut i = 0;
            while i+1 < len {
                if self.bots[i].crash(&self.bots[i+1]) {
                    self.bots.remove(i+1);
                    self.bots.remove(i);
                    len -= 2;
                } else {
                    i += 1;
                }
            }
        }
        let bullets = self.bullets.clone();
        self.bots.retain(|x| x.pos().z > 0. && !bullets.iter().any(|b| x.hit(b)));
        self.bullets = bullets.into_iter().filter(|x| {
            let x = x[0];
            x.x>0. && x.x<self.tunel.size.x &&
            x.y>0. && x.y<self.tunel.size.y &&
            x.z>0. && x.z<self.tunel.size.z
        }).collect();
    }
    pub fn add_bot(&mut self, rules: &BoxRules) {
        self.bots.push_back(Bot::new_random(rules));
    }
    pub fn add_bullet(&mut self, origin: Vector3<f64>, direction: Vector3<f64>, len: f64) {
        self.bullets.push([
            origin,
            direction*len/direction.magnitude(),
            direction,
        ]);
    }
}
