use Pixel;
use world::World;
use piston_window::*;
use camera::Camera;
use cgmath::{Vector3, Vector2};
use cgmath::prelude::*;
use color::*;
use rnd;
use car::*;
use piston_window::Ellipse;
use bot::BoxRules;

pub struct Game {
    config: GameConfig,
    world: World,
    window: PistonWindow,
    bot_rules: BoxRules,
    camera: Camera,
    state: State,
}

struct State {
    pub turn: Turn,
    pub sprint: bool,
    pub spawn: f64,
    pub ended: bool,
    pub game_speed: f64,
    pub jump_timeout: f64,
    pub rotate_cam: bool,
    pub bullets: i64,
    pub recharge: f64,
}

pub enum Turn { Left, Right, None, }

pub struct GameConfig {
    pub title: &'static str,
    pub screen_size: Pixel,
    pub ups: u64,
    pub max_fps: u64,
    pub tunel_size: [f64; 3],
    pub player_size: [f64; 3],
    pub player_speed: (f64, f64),
    pub player_turn_speed: f64,
    pub bot_size: [(f64, f64); 3],
    pub bot_speed: (f64, f64),
    pub bot_turn_speed: (f64, f64),
    pub divider_size: [f64; 2],
    pub camera_height: f64,
    pub camera_distance: f64,
    pub decor_distance: f64,
    pub sprint_factor: f64,
    pub spawn_time: (f64, f64),
    pub game_sprint: f64,
    pub game_max_speed: f64,
    pub player_jump_v: f64,
    pub player_jump_a: f64,
    pub jump_turn_decrease: f64,
    pub jump_timeout: f64,
    pub mouse_speed: f64,
    pub trueshot_distance: f64,
    pub bullet_stock: i64,
    pub recharge_time: f64,
    pub bullet_len: f64,
    pub bullet_speed: f64,
    pub zoom_in: bool,
}

impl Game {
    pub fn new(config: GameConfig) -> Game {
        let mut window: PistonWindow = WindowSettings::new(
            config.title, [config.screen_size.w, config.screen_size.h])
            .exit_on_esc(true).build().unwrap();
        window.set_ups(config.ups);
        window.set_max_fps(config.max_fps);
        window.set_capture_cursor(true);
        let bot_rules = BoxRules {
            size: config.bot_size,
            position: [(0., config.tunel_size[0]), (0., 0.), (config.tunel_size[2], config.tunel_size[2])],
            speed: config.bot_speed,
            turn_speed: config.bot_turn_speed,
            color: vec![RED, ORANGE, VIOLET, GREEN, PALE],
            jump_turn_decrease: config.jump_turn_decrease,
        };
        let world = World::new(&config);
        let camera = Game::new_camera(&config, &world.player);
        let state = State {
            turn: Turn::None,
            sprint: false,
            spawn: 0.,
            ended: false,
            game_speed: 0.,
            jump_timeout: 0.,
            rotate_cam: false,
            bullets: config.bullet_stock,
            recharge: 0.,
        };
        Game {
            config: config,
            world: world,
            window: window,
            bot_rules: bot_rules,
            camera: camera,
            state: state,
        }
    }

    fn new_camera<T: Car>(config: &GameConfig, player: &T) -> Camera {
        Camera::new(
            config.screen_size.clone(),
            Vector3::new(0., config.camera_height, -config.camera_distance) + player.pos()
        )
    }

    pub fn run(&mut self) {
        while let Some(e) = self.window.next() {
            match e {
                Input::Press(key) => self.press(key),
                Input::Release(key) => self.release(key),
                Input::Render(_) => self.draw(&e),
                Input::Update(args) => self.update(args.dt),
                Input::Move(Motion::MouseRelative(a, b)) => self.mouse_move(a as f64, b as f64),
                _ => {}
            }
            if self.state.ended {
                break;
            }
        }
    }

    fn mouse_move(&mut self, x: f64, y: f64) {
        if self.state.rotate_cam {
            self.camera.rotate(x*self.config.mouse_speed, y*self.config.mouse_speed, self.world.player.position);
        }
    }
    fn press(&mut self, key: Button) {
        match key {
            Button::Keyboard(Key::A) => self.state.turn = Turn::Left,
            Button::Keyboard(Key::D) => self.state.turn = Turn::Right,
            Button::Keyboard(Key::W) => self.state.sprint = true,
            Button::Keyboard(Key::Space) => if self.state.jump_timeout <= 0. {
                self.state.jump_timeout = self.config.jump_timeout;
                self.world.player.jump();
            },
            Button::Mouse(MouseButton::Right) => {
                if self.config.zoom_in {
                    self.camera.zoom_in();
                }
                self.state.rotate_cam = true;
            },
            Button::Mouse(MouseButton::Left) => if self.state.rotate_cam && self.state.bullets > 0 {
                let mut pos = self.world.player.position;
                pos.y += self.world.player.size.y;
                let mut d = Vector3::new(0., 0., self.config.trueshot_distance + self.config.camera_distance);
                d = self.camera.c * d.magnitude2() / d.dot(self.camera.c);
                d = self.camera.eye + d - pos;
                d = d * self.config.bullet_speed / d.magnitude();
                self.world.add_bullet(pos, d, self.config.bullet_len);
                self.state.bullets -= 1;
                if self.state.bullets <= 0 {
                    self.state.recharge = self.config.recharge_time;
                }
            },
            _ => (),
        }
    }
    fn release(&mut self, key: Button) {
        match key {
            Button::Keyboard(Key::A) => if let Turn::Left = self.state.turn {
                self.state.turn = Turn::None;
            },
            Button::Keyboard(Key::D) => if let Turn::Right = self.state.turn {
                self.state.turn = Turn::None;
            },
            Button::Keyboard(Key::W) => self.state.sprint = false,
            Button::Mouse(MouseButton::Right) => {
                self.state.rotate_cam = false;
                self.camera = Game::new_camera(&self.config, &self.world.player);
            },
            _ => (),
        }
    }
    fn draw(&mut self, e: &Input) {
        let lines = self.world.render(&self.camera);
        let jump_bar = [
            0.,
            self.config.screen_size.h as f64 - 20.,
            self.config.screen_size.w as f64/2.*self.state.jump_timeout/self.config.jump_timeout,
            self.config.screen_size.h as f64,
        ];
        let recharge_bar = [
            0.,
            self.config.screen_size.h as f64 - 20.,
            self.config.screen_size.w as f64/2.*self.state.recharge/self.config.recharge_time,
            self.config.screen_size.h as f64,
        ];
        let bullets_bar = [
            0.,
            self.config.screen_size.h as f64 - 20.,
            self.config.screen_size.w as f64/2.*self.state.bullets as f64/self.config.bullet_stock as f64,
            self.config.screen_size.h as f64,
        ];
        self.window.draw_2d(e, |c, g| {
            clear(BLACK, g);
            for (l, color) in lines {
                line(color, 1., convert(l), c.transform, g);
            }
            rectangle(pale(BLUE, 0.4), jump_bar, c.transform, g);
            rectangle(pale(RED, 0.4), recharge_bar, c.transform, g);
            rectangle(pale(GREEN, 0.4), bullets_bar, c.transform, g);
        });

        if self.state.rotate_cam {
            let w = 20.;
            let x = self.config.screen_size.w as f64 /2. - w/2.;
            let y = self.config.screen_size.h as f64 /2. - w/2.;
            self.window.draw_2d(e, |c, g| {
                let e = Ellipse {
                    color: pale(BLACK, 0.),
                    border: Some(ellipse::Border {
                        color: pale(RED, 0.5),
                        radius: 1.,
                    }),
                    resolution: 360,
                };
                e.draw([x, y, w, w], &c.draw_state, c.transform, g);
                rectangle(RED, [x+w/2.-1., y+w/2.-1., 2., 2.], c.transform, g);
            });
        }

    }
    fn update(&mut self, dt: f64) {
        let old = self.world.player.position;
        if self.state.bullets <= 0 {
            self.state.recharge -= dt;
            if self.state.recharge < 0. {
                self.state.bullets = self.config.bullet_stock;
            }
        }
        self.state.jump_timeout -= dt;
        if self.state.game_speed < self.config.game_max_speed {
            self.state.game_speed += dt*self.config.game_sprint;
        }
        if self.state.sprint {
            if self.world.player.speed < self.config.player_speed.1 {
                self.world.player.speed += dt*self.config.sprint_factor;
            }
        } else if self.world.player.speed > self.config.player_speed.0 {
            self.world.player.speed -= dt*self.config.sprint_factor;
        }
        self.state.spawn -= dt;
        if self.state.spawn < 0. {
            self.world.add_bot(&self.bot_rules);
            self.state.spawn += rnd(self.config.spawn_time);
        }
        match self.state.turn {
            Turn::Left => self.world.player.turn_left(dt),
            Turn::Right => self.world.player.turn_right(dt),
            Turn::None => (),
        }
        self.world.update(dt, self.state.game_speed);
        self.world.validate();
        self.camera.eye += self.world.player.position - old;
        for ref x in &self.world.bots {
            if self.world.player.crash(&x.car) {
                self.state.ended = true;
            }
        }
    }
}

fn convert(x: [Vector2<f64>; 2]) -> [f64; 4] {
    [x[0].x, x[0].y, x[1].x, x[1].y]
}
