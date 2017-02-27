use car::*;
use cgmath::{Vector2, Vector3};
use color::*;
use Color;
use camera::Camera;
use rnd;
use rand;

pub enum Action {
    Rest(f64),
    Jump,
    TurnLeft(f64),
    TurnRight(f64),
}

pub struct Bot {
    pub car: BoxCar,
    pub actions: Vec<Action>,
}

impl Bot {
     pub fn new_random(rules: &BoxRules) -> Bot {
         Bot{
             car: BoxCar {
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
             },
             actions: Vec::new(),
         }
    }
}


impl Car for Bot {
    fn render(&self, cam: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        self.car.render(cam)
    }
    fn crash(&self, x: &Self) -> bool {
        self.car.crash(&x.car)
    }
    fn hit(&self, x: &[Vector3<f64>; 3]) -> bool {
        self.car.hit(x)
    }
    fn forward(&mut self, x: f64, y: f64) {
        self.car.forward(x, y)
    }
    fn turn_left(&mut self, x: f64) {
        self.car.turn_left(x)
    }
    fn turn_right(&mut self, x: f64) {
        self.car.turn_right(x)
    }
    fn update_jump(&mut self, x: f64) {
        self.car.update_jump(x)
    }
    fn jump(&mut self) {
        self.car.jump()
    }
    fn pos(&self) -> Vector3<f64> {
        self.car.pos()
    }
    fn turn_speed(&self) -> f64 {
        self.car.turn_speed()
    }
}
