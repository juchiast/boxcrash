#[derive(Copy, Clone)]
pub struct Color([f32; 4]);

impl Color {
    // Set alpha chanel `a` for color
    pub fn alpha(mut self, a: f32) -> Self {
        (self.0)[3] = a;
        self
    }
}

impl Into<Color> for [f32; 4] {
    fn into(self) -> Color {
        Color(self)
    }
}
impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        self.0
    }
}
impl Into<::conrod::Color> for Color {
    fn into(self) -> ::conrod::Color {
        let c = self.0;
        ::conrod::Color::Rgba(c[0], c[1], c[2], c[3])
    }
}

pub const BLACK: Color = Color([0.0, 0.0, 0.0, 1.0]);
pub const GREEN: Color = Color([0.0, 1.0, 0.0, 1.0]);
pub const BLUE: Color = Color([0.0, 0.0, 1.0, 1.0]);
pub const ORANGE: Color = Color([1.0, 0.5, 0.0, 1.0]);
pub const RED: Color = Color([1.0, 0.0, 0.0, 1.0]);
pub const VIOLET: Color = Color([0.6, 0.0, 1.0, 1.0]);
pub const YELLOW: Color = Color([1.0, 1.0, 0.0, 1.0]);
pub const WHITE: Color = Color([1.0, 1.0, 1.0, 1.0]);
pub const PALE: Color = Color([0.3, 0.3, 0.3, 0.1]);
pub const GRAY: Color = Color([0.6, 0.6, 0.6, 1.0]);
