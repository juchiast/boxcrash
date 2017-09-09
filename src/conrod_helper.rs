use piston_window::{G2d, UpdateEvent, Input, Window, PistonWindow, G2dTexture, TextureSettings};
use piston_window::texture::UpdateTexture;
use piston_window;
use conrod::{UiBuilder, Ui, UiCell};
use conrod::widget;
use conrod::text::GlyphCache;
use conrod;

use Pixel;
use control::{Flow, EventHandler};

pub trait Gui {
    type Ids;

    fn new() -> Self;
    fn gui(&mut self, &mut UiCell, &Self::Ids) -> Option<Flow>;
    fn ids(&self, widget::id::Generator) -> Self::Ids;
}

pub struct ConrodUI<G: Gui> {
    gui: G,
    ids: G::Ids,
    ui: Ui,
    glyph_cache: GlyphCache,
    text_texture_cache: G2dTexture,
}

impl<G: Gui> ConrodUI<G> {
    pub fn new(size: Pixel, window: &mut PistonWindow) -> ConrodUI<G> {
        let gui = G::new();
        let mut ui = UiBuilder::new([size.w as f64, size.h as f64])
            .theme(theme())
            .build();

        ui.fonts.insert_from_file("resources/Ubuntu-R.ttf")
            .expect("Cannot read font.");

        // Create a texture to use for efficiently caching text on the GPU.
        let (glyph_cache, text_texture_cache) = {
            const SCALE_TOLERANCE: f32 = 0.1;
            const POSITION_TOLERANCE: f32 = 0.1;
            let cache = GlyphCache::new(size.w, size.h, SCALE_TOLERANCE, POSITION_TOLERANCE);
            let buffer_len = size.w as usize * size.h as usize;
            let init = vec![128; buffer_len];
            let settings = TextureSettings::new();
            let factory = &mut window.factory;
            let texture = G2dTexture::from_memory_alpha(factory, &init, size.w, size.h, &settings).unwrap();
            (cache, texture)
        };  

        let ids = gui.ids(ui.widget_id_generator());

        ConrodUI {
            gui: gui,
            ids: ids,
            ui: ui,
            glyph_cache: glyph_cache,
            text_texture_cache: text_texture_cache,
        }
    }
}

impl<G: Gui> EventHandler for ConrodUI<G> {
    fn handle_event(&mut self, event: Input, window: &mut PistonWindow) -> Option<Flow> {
        let size = window.size();
        let (win_w, win_h) = (size.width as f64, size.height as f64);
        if let Some(e) = conrod::backend::piston::event::convert(event.clone(), win_w, win_h) {
            self.ui.handle_event(e);
        }

        let mut flow = None;
        event.update(|_| {
            let mut ui = self.ui.set_widgets();
            flow = self.gui.gui(&mut ui, &self.ids);
        });

        window.draw_2d(&event, |context, graphics| {
            if let Some(primitives) = self.ui.draw_if_changed() {
                let cache_queued_glyphs = |graphics: &mut G2d,
                cache: &mut G2dTexture,
                rect: conrod::text::rt::Rect<u32>,
                data: &[u8]|
                {
                    let offset = [rect.min.x, rect.min.y];
                    let size = [rect.width(), rect.height()];
                    let format = piston_window::texture::Format::Rgba8;
                    let encoder = &mut graphics.encoder;
                    let text_vertex_data: Vec<_> = data.iter().flat_map(|&b| vec![255, 255, 255, b]).collect();
                    UpdateTexture::update(cache, encoder, format, &text_vertex_data[..], offset, size)
                        .expect("failed to update texture");
                };

                fn texture_from_image<T>(img: &T) -> &T { img }

                conrod::backend::piston::draw::primitives(primitives,
                                                          context,
                                                          graphics,
                                                          &mut self.text_texture_cache,
                                                          &mut self.glyph_cache,
                                                          &conrod::image::Map::new(),
                                                          cache_queued_glyphs,
                                                          texture_from_image);
            }
        });

        flow
    }
}

fn theme() -> conrod::Theme {
    use conrod::position::{Padding, Position};
    use std;
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Absolute(10.0),
        y_position: Position::Absolute(10.0),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}