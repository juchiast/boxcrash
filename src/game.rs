use Pixel;
use world::World;
use piston_window::*;
use opengl_graphics::GlGraphics;
use car::CarRules;
use tunel::Tunel;

pub struct Game {
    config: GameConfig,
    world: World,
    window: PistonWindow,
    opengl: OpenGL,
    bot_rules: CarRules,
}

pub struct GameConfig {
    pub title: &'static str,
    pub screen_size: Pixel,
    pub ups: u64,
    pub max_fps: u64,
    pub tunel_size: [f64; 3],
}

impl Game {
    pub fn new(config: GameConfig) -> Game {
        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow = WindowSettings::new(
            config.title, [config.screen_size.w, config.screen_size.h])
            .opengl(opengl).exit_on_esc(true).build().unwrap();
        window.set_ups(config.ups);
        window.set_max_fps(config.max_fps);
        let bot_rules = CarRules {
            size: [(1., 2.), (1.5, 2.), (3., 4.)],
            position: [(0., config.tunel_size[0]), (0., 0.), (0., config.tunel_size[2])],
            speed: (5., 10.),
            turn_speed: (0., 0.),
            color: Vec::new(),
        };
        let world = World::new(&config);
        Game {
            config: config,
            world: world,
            window: window,
            opengl: opengl,
            bot_rules: bot_rules,
        }
    }

    pub fn run(&mut self) {
        let mut gl = GlGraphics::new(self.opengl);
        while let Some(e) = self.window.next() {
            match e {
                Input::Press(Button::Keyboard(key)) => self.key_press(key),
                Input::Release(Button::Keyboard(key)) => self.key_release(key),
                Input::Render(args) => {
                    gl.draw(args.viewport(), |c, g| self.draw(c, g));
                },
                Input::Update(args) => self.update(args.dt),
                _ => {}
            }
        }
    }

    fn key_press(&mut self, key: Key) {}
    fn key_release(&mut self, key: Key) {}
    fn draw(&mut self, c: Context, g: &mut GlGraphics) {}
    fn update(&mut self, dt: f64) {}
}
