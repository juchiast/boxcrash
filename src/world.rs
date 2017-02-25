use tunel::Tunel;
use car::Car;
use Color;
use color::*;
use cgmath::{Vector2, Vector3};
use game::GameConfig;
use camera::Camera;

pub struct World {
    pub tunel: Tunel,
    pub player: Car,
    pub bots: Vec<Car>,
    pub divider: Vector2<f64>,
    pub decor_distance: f64,
    pub divider_state: f64,
    pub decor_state: f64,
}

impl World {
    pub fn new(config: &GameConfig) -> World {
        let player = Car {
            size: Vector3::from(config.player_size),
            position: Vector3::new(config.tunel_size[0]/2., 0., 10.),
            speed: config.player_speed,
            turn_speed: config.player_turn_speed,
            color: YELLOW,
        };

        World {
            tunel: Tunel::new(config.tunel_size),
            player: player,
            bots: Vec::new(),
            divider: Vector2::from(config.divider_size),
            divider_state: config.divider_size[1],
            decor_distance: config.decor_distance,
            decor_state: config.decor_distance,
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
        ret
    }
    fn divider_render(&self, camera: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        let mut points = [Vector3::new(self.tunel.size.x/2., 0., self.divider_state); 4];
        points[2].z -= self.divider.y; points[3].z -= self.divider.y;
        points[0].x -= self.divider.x/2.; points[3].x -= self.divider.x/2.;
        points[1].x += self.divider.x/2.; points[2].x += self.divider.x/2.;
        
        let mut ret = Vec::new();
        while points[0].z <= self.tunel.size.z {
            for i in 0..4 {
                if let Some(rendered) = camera.render_line(&points[i], &points[(i+1)%4]) {
                    ret.push((rendered, self.tunel.color));
                }
            }
            for i in 0..4 {
                points[i].z += 2.*self.divider.y;
            }
        }
        ret
    }
    fn decor_render(&self, camera: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        let mut data = [
            Vector3::new(0., 0., self.decor_state),
            Vector3::new(0., self.tunel.size.y, self.decor_state),
            Vector3::new(self.tunel.size.x, self.tunel.size.y, self.decor_state),
            Vector3::new(self.tunel.size.x, 0., self.decor_state),
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
    pub fn update(&mut self, dt: f64) {
        if self.divider_state < 0. {
            self.divider_state += 2.*self.divider.y;
        } else {
            self.divider_state -= dt*self.player.speed;
        }
        if self.decor_state < 0. {
            self.decor_state += self.decor_distance;
        } else {
            self.decor_state -= dt*self.player.speed;
        }
    }
    pub fn validate(&mut self) {
        if self.player.position.x + self.player.size.x/2. > self.tunel.size.x {
            self.player.position.x = self.tunel.size.x - self.player.size.x/2.;
        } else if self.player.position.x - self.player.size.x/2. < 0. {
            self.player.position.x = self.player.size.x/2.;
        }
    }
}
