use gi_ui::canvas::Canvas;
use gi_ui::components::circle::Circle;
use gi_ui::draw::Draw;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT); // 100, 100
    canvas.fill(0xff_000000);

    let mut circle = Circle::new()
        .with_radius(25)
        .set_border(0xff_ffffff, 5)
        .set_foreground_color(0xff_0000aa);

    circle.draw(&mut canvas, 0, 0);

    gi_ui::helpers::export_to_png(&canvas);
}
