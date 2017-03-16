use conrod;
use conrod::{Positionable, Sizeable, Labelable, Widget};
use conrod::widget;
use conrod::backend::glium::{glium, Renderer};
use conrod::backend::winit;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use support;
use ::game::{GameConfig, Game};


pub fn start_game(config: GameConfig) {
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(300, 300)
        .with_title("Test")
        .build_glium()
        .expect("Cannot load UI");
    let mut ui = conrod::UiBuilder::new([300., 300.]).build();
    widget_ids!(struct Ids { canvas, what });
    let ids = Ids::new(ui.widget_id_generator());
    ui.fonts.insert_from_file("resources/Ubuntu-R.ttf").expect("UI cannot load font");
    let mut renderer = Renderer::new(&display).expect("UI cannot load renderer");
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut event_loop = support::EventLoop::new();
    'main: loop {
        for event in event_loop.next(&display) {
            if let Some(event) = winit::convert(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }
            if let glium::glutin::Event::Closed = event {
                break 'main;
            }
            {
                let ui = &mut ui.set_widgets();
                widget::Canvas::new().set(ids.canvas, ui);
                for _click in widget::Button::new()
                    .middle_of(ids.canvas)
                    .w_h(120., 30.)
                    .label("Start game")
                    .set(ids.what, ui)
                {
                    Game::new(config.clone()).run();
                }
            }
            if let Some(whatever) = ui.draw_if_changed() {
                renderer.fill(&display, whatever, &image_map);
                let mut target = display.draw();
                target.clear_color(0., 0., 0., 1.);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }
}
