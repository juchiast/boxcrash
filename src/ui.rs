use conrod;
use conrod::{Positionable, Sizeable, Labelable, Widget, Colorable};
use conrod::widget;
use conrod::backend::glium::{glium, Renderer};
use conrod::backend::winit;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use support;
use ::game::{GameConfig, Game};
use std::thread;

pub fn main(config: &mut GameConfig) {
    let mut first_time = true;
    while let Some(th) = start_game(config, first_time) {
        th.join().expect("Unexpected thread error");
        first_time = false;
    }
}

fn start_game(config: &mut GameConfig, first_time: bool) -> Option<thread::JoinHandle<()>> {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(320, 240)
        .with_title(config.title.as_ref())
        .build_glium()
        .expect("UI: Cannot build window");
    let mut ui = conrod::UiBuilder::new([320., 240.]).build();
    ui.fonts.insert_from_file("resources/Ubuntu-R.ttf").expect("UI: cannot load font");
    let mut renderer = Renderer::new(&display).expect("UI: cannot load renderer");
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut event_loop = support::EventLoop::new();

    widget_ids!(struct Ids {
        canvas,
        button,
        w_label, h_label,
        w_text_box, h_text_box,
    });
    let ids = Ids::new(ui.widget_id_generator());
    let mut screen_w = config.screen_size.w.to_string();
    let mut screen_h = config.screen_size.h.to_string();
    loop {
        for event in event_loop.next(&display) {
            if let Some(event) = winit::convert(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }
            if let glium::glutin::Event::Closed = event { return None; }

            {
                let ui = &mut ui.set_widgets();
                let _canvas = widget::Canvas::new()
                    .rgb(0.5, 0.5, 0.5)
                    .set(ids.canvas, ui);
                let w_text_box = widget::TextBox::new(&screen_w)
                    .mid_top_of(ids.canvas)
                    .w_h(120., 30.)
                    .font_size(14)
                    .set(ids.w_text_box, ui);
                let h_text_box = widget::TextBox::new(&screen_h)
                    .down_from(ids.w_text_box, 2.)
                    .w_h(120., 30.)
                    .font_size(14)
                    .set(ids.h_text_box, ui);
                let _w_label = widget::Text::new("Width: ")
                    .left_from(ids.w_text_box, 0.).align_middle_y_of(ids.w_text_box)
                    .font_size(14).rgb(1., 1., 1.)
                    .set(ids.w_label, ui);
                let _h_label = widget::Text::new("Height: ")
                    .left_from(ids.h_text_box, 0.).align_middle_y_of(ids.h_text_box)
                    .font_size(14).rgb(1., 1., 1.)
                    .set(ids.h_label, ui);
                let button = widget::Button::new()
                    .down_from(ids.h_text_box, 2.)
                    .w_h(120., 30.)
                    .label(if first_time {"Start game"} else {"Restart game"})
                    .label_font_size(14)
                    .set(ids.button, ui);

                for e in h_text_box {
                    if let widget::text_box::Event::Update(s) = e {
                        if s.is_empty() || s.parse::<u32>().is_ok() {
                            screen_h = s;
                        }
                    }
                }
                for e in w_text_box {
                    if let widget::text_box::Event::Update(s) = e {
                        if s.is_empty() || s.parse::<u32>().is_ok() {
                            screen_w = s;
                        }
                    }
                }
                for _click in button {
                    config.screen_size.w = screen_w.parse().unwrap_or(config.screen_size.w);
                    config.screen_size.h = screen_h.parse().unwrap_or(config.screen_size.h);
                    let config = config.clone();
                    return Some(thread::spawn(|| Game::new(config).run()));
                }
            }

            if let Some(whatever) = ui.draw_if_changed() {
                renderer.fill(&display, whatever, &image_map);
                let mut target = display.draw();
                target.clear_color(0., 0., 0., 1.);
                renderer.draw(&display, &mut target, &image_map).expect("UI draw error");
                target.finish().expect("UI 'finish' error");
            }
        }
    }
}
