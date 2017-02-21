use Color;
use Pixel;

enum Shape {
    Point(Pixel),
    Line(Pixel, Pixel),
}

struct Graphics {
    shape: Shape,
    color: Color,
}

trait Drawable {
    fn draw();
}

impl Drawable for Vec<Graphics> {
    fn draw() {}
}
