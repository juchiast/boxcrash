use cgmath::{Vector2, Vector3, vec3};
use crate::color::*;
use super::camera::Camera;
use super::car::*;
use crate::rnd;

// Action with duration
// e.g. `TurnLeft(3.)` means turn left for 3s
#[derive(Clone)]
pub enum Action {
    Rest(f64), // Do nothing
    Jump,
    TurnLeft(f64),
    TurnRight(f64),
}

// Rules to generate new bot
pub struct BoxRules {
    pub size: [(f64, f64); 3],
    pub position: [(f64, f64); 3],
    pub speed: (f64, f64),
    pub turn_speed: (f64, f64),
    pub color: Vec<Color>,
    pub jump_turn_decrease: f64,
}

#[derive(Clone)]
pub struct Bot {
    pub car: BoxCar,
    pub actions: Vec<Action>,
}

impl Action {
    fn rand() -> Action {
        match ::rand::random::<usize>() % 4 {
            0 => Action::Rest(rnd((0.25, 1.))),
            1 => Action::TurnLeft(rnd((0.25, 1.))),
            2 => Action::TurnRight(rnd((0.25, 1.))),
            3 => Action::Jump,
            _ => panic!("Unexpected error in `Action::rand()`"),
        }
    }
}

impl Bot {
    pub fn new_random(rules: &BoxRules) -> Bot {
        Bot {
            car: BoxCar {
                size: vec3(rnd(rules.size[0]), rnd(rules.size[1]), rnd(rules.size[2])),
                position: vec3(
                    rnd(rules.position[0]),
                    rnd(rules.position[1]),
                    rnd(rules.position[2]),
                ),
                speed: rnd(rules.speed),
                turn_speed: rnd(rules.turn_speed),
                color: if rules.color.is_empty() {
                    RED
                } else {
                    rules.color[::rand::random::<usize>() % rules.color.len()]
                },
                jump_v: 5.,
                jump_a: 7.,
                jumping: false,
                current_t: 0.,
                jump_turn_decrease: rules.jump_turn_decrease,
            },
            actions: (0..::rand::random::<usize>() % 6)
                .map(|_| Action::rand())
                .collect(),
        }
    }
    pub fn drive(&mut self, dt: f64) {
        if let Some(a) = self.actions.pop() {
            match a {
                Action::Jump => self.car.jump(),
                Action::TurnRight(t) => if t > 0. {
                    self.car.turn_right(dt);
                    self.actions.push(Action::TurnRight(t - dt));
                },
                Action::TurnLeft(t) => if t > 0. {
                    self.car.turn_left(dt);
                    self.actions.push(Action::TurnLeft(t - dt));
                },
                Action::Rest(t) => if t > 0. {
                    self.actions.push(Action::Rest(t - dt));
                },
            }
        }
    }
}

// Bot is also a `Car`
impl Car for Bot {
    fn render(&self, cam: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        self.car.render(cam)
    }
    fn crashed(&self, x: &Self) -> bool {
        self.car.crashed(&x.car)
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
