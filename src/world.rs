use tunel::Tunel;
use car::*;
use color::*;
use cgmath::{Vector2, Vector3, vec3};
use cgmath::prelude::*;
use game::GameConfig;
use camera::Camera;
use bot::{Bot, BoxRules};

pub struct World {
    pub tunel: Tunel,
    pub player: BoxCar,
    pub bots: Vec<Bot>,
    pub divider: Vector2<f64>,
    pub decor_distance: f64,
    pub divider_state: f64,
    pub decor_state: f64,
    pub bullets: Vec<[Vector3<f64>; 3]>,
}
type Rendered = Vec<([Vector2<f64>; 2], Color)>;
impl World {
    fn divider_render(&self, camera: &Camera) -> Rendered {
        let mut points = [vec3(self.tunel.size.x/2., 0., self.divider_state); 4];
        points[2].z -= self.divider.y; points[3].z -= self.divider.y;
        points[0].x -= self.divider.x/2.; points[3].x -= self.divider.x/2.;
        points[1].x += self.divider.x/2.; points[2].x += self.divider.x/2.;
        
        let mut ret = Vec::new();
        {
            let mut r = |p: &[Vector3<f64>; 4]| {
                let iter = p.iter().zip(p.iter().cycle().skip(1))
                    .map(|(x, y)| camera.render_line(x, y))
                    .filter_map(|x| x.map(|x| (x, self.tunel.color)));
                ret.append(&mut iter.collect());
            };
            while points[0].z <= self.tunel.size.z {
                r(&points);
                for p in &mut points {
                    p.z += 2.*self.divider.y;
                }
            }
            r(&points);
        }
        ret
    }
    fn decor_render(&self, camera: &Camera) -> Rendered {
        let mut data = [
            vec3(0., 0., self.decor_state),
            vec3(0., self.tunel.size.y, self.decor_state),
            vec3(self.tunel.size.x, self.tunel.size.y, self.decor_state),
            vec3(self.tunel.size.x, 0., self.decor_state),
        ];
        let mut ret = Vec::new();
        while data[0].z <= self.tunel.size.z {
            for (x, y) in data.iter().zip(data.iter().skip(1)) {
                if let Some(rendered) = camera.render_line(x, y) {
                    ret.push((rendered, self.tunel.color));
                }
            }
            for x in &mut data {
                x.z += self.decor_distance;
            }
        }
        ret
    }
    fn bullets_render(&self, camera: &Camera) -> Rendered {
        self.bullets.iter().filter_map(|x| {
            camera.render_line(&x[0], &(x[0]+x[1])).map(|x| (x, self.player.color))
        }).collect()
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
            bots: Vec::new(),
            divider: config.divider_size.into(),
            divider_state: config.divider_size[1],
            decor_distance: config.decor_distance,
            decor_state: config.decor_distance,
            bullets: Vec::new(),
        }
    }

    pub fn render(&self, camera: &Camera) -> Rendered {
        Vec::new().into_iter()
            .chain(self.tunel.render(camera))
            .chain(self.divider_render(camera))
            .chain(self.decor_render(camera))
            .chain(self.player.render(camera))
            .chain(self.bots.iter().flat_map(|x| x.render(camera)))
            .chain(self.bullets_render(camera))
            .collect()
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
        for x in &mut self.bots {
            x.drive(dt);
            x.forward(dt, speed);
        }
        for x in &mut self.bullets {
            x[0] += dt*x[2];
        }
    }
    pub fn validate(&mut self) {
        let size = self.tunel.size;
        self.bullets.retain(|x| {
            let x = x[0];
            x.x>0. && x.x<size.x &&
            x.y>0. && x.y<size.y &&
            x.z>0. && x.z<size.z
        });

        let validate_car = |car: &mut BoxCar| {
            if car.position.x + car.size.x/2. > size.x {
                car.position.x = size.x - car.size.x/2.;
            } else if car.position.x - car.size.x/2. < 0. {
                car.position.x = car.size.x/2.;
            }
            if car.position.y + car.size.y > size.y {
                car.position.y = size.y - car.size.y;
            }
        };
        validate_car(&mut self.player);
        for x in &mut self.bots {
            validate_car(&mut x.car);
        }

        let bullets = self.bullets.clone();
        self.bots.retain(|x| x.pos().z > 0. && !bullets.iter().any(|b| x.hit(b)));
        self.bots.sort_by(|a, b| a.pos().z.partial_cmp(&b.pos().z).unwrap());
        let set = self.bots.iter().zip(self.bots.iter().skip(1)).enumerate()
            .filter(|&(_, (x, y))| x.crashed(y)).map(|(i, _)| i)
            .collect::<::std::collections::BTreeSet<_>>();
        self.bots = self.bots.iter().enumerate()
            .filter(|&(i, _)| !( (i>0 && set.contains(&(i-1))) || set.contains(&i) ))
            .map(|(_, x)| x.clone()).collect();
    }
    pub fn add_bot(&mut self, rules: &BoxRules) {
        self.bots.push(Bot::new_random(rules));
    }
    pub fn add_bullet(&mut self, origin: Vector3<f64>, direction: Vector3<f64>, len: f64) {
        self.bullets.push([
            origin,
            direction*len/direction.magnitude(),
            direction,
        ]);
    }
}
