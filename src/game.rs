use world::World;
use camera::Camera;
use color::*;
use car::*;
use bot::BoxRules;
use Pixel;
use control::{Flow, EventHandler};

use std::cell::RefCell;
use std::ops::DerefMut;
use std::time::Instant;

use piston_window::*;
use cgmath::{Vector2, vec3};
use cgmath::prelude::*;

// Configurable game's contansts.
// A tuple presents a range of something.
#[derive(Serialize, Deserialize, Clone)]
pub struct GameConfig {
    pub title: String,
    pub screen_size: ::Pixel,
    pub ups: u64, // Update per second
    pub max_fps: u64,
    pub tunel_size: [f64; 3],
    pub player_size: [f64; 3],
    pub player_speed: (f64, f64), // min and max player speed
    pub player_turn_speed: f64,
    pub bot_size: [(f64, f64); 3], // Range of bot's size
    pub bot_speed: (f64, f64),
    pub bot_turn_speed: (f64, f64),
    pub divider_size: [f64; 2],
    pub camera_height: f64, // Height of camera (from player)
    pub camera_distance: f64, // Distance from camera to player
    pub decor_distance: f64, // Distance between each decoration
    pub sprint_factor: f64,
    pub spawn_time: (f64, f64),
    pub game_sprint: f64, // The increase of game_speed
    pub game_max_speed: f64,
    pub player_jump_v: f64,
    pub player_jump_a: f64,
    pub jump_turn_decrease: f64,
    pub jump_timeout: f64,
    pub mouse_speed: f64,
    pub trueshot_distance: f64,
    pub bullet_stock: i64, // Number of bullets
    pub recharge_time: f64,
    pub bullet_len: f64,
    pub bullet_speed: f64,
    pub zoom_in: bool, // If true, zoom-in while on stare mode
}

impl Default for GameConfig {
    fn default() -> GameConfig {
        use std::f64::consts::PI;
        GameConfig {
            title: "Box Crash".to_owned(),
            screen_size: Pixel::new(800, 600),
            ups: 60,
            max_fps: 60,
            tunel_size: [15., 8., 150.],
            player_size: [1.5, 0.8, 3.],
            player_speed: (20., 120.),
            player_turn_speed: 15.,
            bot_size: [(1., 4.), (0.5, 2.5), (2.5, 8.)],
            bot_speed: (20., 120.),
            bot_turn_speed: (5., 20.),
            divider_size: [1., 7.],
            camera_height: 3.,
            camera_distance: 5.5,
            decor_distance: 8.,
            sprint_factor: 15.,
            spawn_time: (0.25, 1.),
            game_sprint: 1.,
            game_max_speed: 80.,
            player_jump_v: 7.,
            player_jump_a: 5.,
            jump_turn_decrease: 3.,
            jump_timeout: 8.,
            mouse_speed: PI/420.,
            trueshot_distance: 100.,
            bullet_stock: 15,
            recharge_time: 10.,
            bullet_len: 5.,
            bullet_speed: 100.,
            zoom_in: false,
        }
    }
}

// `Game` contains every things to run the game
pub struct Game {
    config: GameConfig,
    world: World, // All objects in the game
    bot_rules: BoxRules, // Rules to create a new bot
    camera: Camera, // Camera for rendering
    state: State, // Current state of game
    // Wrap these caches in `RefCell` to allow interior mutability
    glyphs: RefCell<Glyphs>, // Font cache
    ellipse: RefCell<Ellipse>, // Model to draw a circle
}

struct State {
    pub turn: Turn, // Presents movement of player
    pub sprint: bool, // Player is speeding-up or not
    pub spawn: f64, // Count down time to spawn a new bot
    pub ended: bool, // Game is over or not
    pub game_speed: f64, // Game speed in addition to player's speed
    pub jump_timeout: f64, // Count down to allow the next jump
    pub rotate_cam: bool, // Allow rotation of camera or not
    pub bullets: i64, // The number of bullets left
    pub recharge: f64, // Bullets recharge time
    pub fps: f64, // Real fps of game
    pub last_frame: Instant, // Moment of the last draw
}

pub enum Turn { Left, Right, None, }

impl Game {
    pub fn new(config: GameConfig, window: &PistonWindow) -> Game {
        let glyphs = Glyphs::new("resources/Ubuntu-R.ttf", window.factory.clone(), texture::TextureSettings::new())
            .expect("Unable to load font.");
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
            fps: 0.,
            last_frame: Instant::now(),
        };
        let ellipse = Ellipse {
            color: BLACK.alpha(0.).into(),
            border: Some(ellipse::Border {
                color: RED.alpha(0.5).into(),
                radius: 1.,
            }),
            resolution: 8,
        };

        Game {
            config: config,
            world: world,
            bot_rules: bot_rules,
            camera: camera,
            state: state,
            glyphs: RefCell::new(glyphs),
            ellipse: RefCell::new(ellipse),
        }
    }

    fn new_camera<T: Car>(config: &GameConfig, player: &T) -> Camera {
        Camera::new(
            config.screen_size.clone(),
            vec3(0., config.camera_height, -config.camera_distance) + player.pos()
        )
    }
    // Re-calculate fps
    fn update_fps(&mut self) {
        let d = self.state.last_frame.elapsed();
        self.state.last_frame = Instant::now();
        self.state.fps = 1. / (d.as_secs() as f64 + 1e-9*d.subsec_nanos() as f64);
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
                let mut d = vec3(0., 0., self.config.trueshot_distance + self.config.camera_distance);
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
    fn draw(&mut self, e: &Event, window: &mut PistonWindow) {
        // Return a horizontal bar
        macro_rules! bar {
            ($curr: expr, $full: expr) => {
                [0.,
                15.0,
                self.config.screen_size.w as f64/2.*$curr/$full,
                20.0,]
            };
        }
        let jump_bar = bar!(self.state.jump_timeout, self.config.jump_timeout);
        let recharge_bar = bar!(self.state.recharge, self.config.recharge_time);
        let bullets_bar = bar!(self.state.bullets as f64, self.config.bullet_stock as f64);
        // Closure in `draw_2d` requires unique access to `self`,
        // so we use RefCell to hack it.
        let mut glyphs = self.glyphs.borrow_mut();
        let fps = format!("{:.3}", self.state.fps);
        let lines = self.world.render(&self.camera);
        window.draw_2d(e, |c, g| {
            clear(BLACK.into(), g);
            for (l, color) in lines {
                line(color.into(), 1., convert(l), c.transform, g);
            }
            rectangle(BLUE.alpha(0.4).into(), jump_bar, c.transform, g);
            rectangle(RED.alpha(0.4).into(), recharge_bar, c.transform, g);
            rectangle(GREEN.alpha(0.4).into(), bullets_bar, c.transform, g);
            text(WHITE.into(), 10, &fps, glyphs.deref_mut(), c.transform.trans(0., 10.), g);
        });

        if self.state.rotate_cam {
            let w = 20.;
            let x = self.config.screen_size.w as f64 /2. - w/2.;
            let y = self.config.screen_size.h as f64 /2. - w/2.;
            let ellipse = self.ellipse.borrow();
            window.draw_2d(e, |c, g| {
                ellipse.draw([x, y, w, w], &c.draw_state, c.transform, g);
                rectangle(RED.into(), [x+w/2.-1., y+w/2.-1., 2., 2.], c.transform, g);
            });
        }
    }
    // `dt` stands for delta, duration since the last update
    fn update(&mut self, dt: f64) {
        // Re-calculate delta according to fps
        let dt = if self.state.fps != 0. { 1./self.state.fps}
        else { dt };
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
            self.state.spawn += ::rnd(self.config.spawn_time);
        }
        match self.state.turn {
            Turn::Left => self.world.player.turn_left(dt),
            Turn::Right => self.world.player.turn_right(dt),
            Turn::None => (),
        }
        // Update objects in the world
        self.world.update(dt, self.state.game_speed);
        // Validate things like object's boundary, bullets and boxes
        // collisions.
        self.world.validate();
        // Update camera's location
        self.camera.eye += self.world.player.position - old;
        // Check for player's collision with bot
        if self.world.bots.iter().any(|x| self.world.player.crashed(&x.car)) {
            self.state.ended = true;
        }
    }
}

impl EventHandler for Game {
    type Input = ();
    fn handle_event(&mut self, e: Event, window: &mut PistonWindow, _: &mut Self::Input) -> Option<Flow> {
        use Input::*;
        use Loop::*;
        use Motion::*;
        use Event::*;
        match e {
            Loop(Render(_)) => {
                self.update_fps();
                self.draw(&e, window);
            },
            Loop(Update(args)) => self.update(args.dt),
            Input(Button(args)) => {
                use ButtonState::*;
                match args.state {
                    Press => self.press(args.button),
                    Release => self.release(args.button),
                }
            },
            Input(Move(MouseRelative(a, b))) => self.mouse_move(a as f64, b as f64),
            _ => {}
        }

        if self.state.ended {
            Some(Flow::LoseGame)
        } else {
            None
        }
    }
}

fn convert(x: [Vector2<f64>; 2]) -> [f64; 4] {
    [x[0].x, x[0].y, x[1].x, x[1].y]
}
