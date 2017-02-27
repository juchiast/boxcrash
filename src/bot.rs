use car::{CarRules, Car};
use cgmath::{Vector2, Vector3};
use color::*;
use rnd;
use rand;

pub enum Action {
    Rest(f64),
    Jump,
    TurnLeft(f64),
    TurnRight(f64),
}

pub struct Bot {
    pub car: Car,
    pub actions: Vec<Action>,
}

impl Bot {
     pub fn new_random(rules: &CarRules) -> Bot {
         Bot{
             car: Car {
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

