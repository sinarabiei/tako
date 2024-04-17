use tako::prelude::*;

fn main() {
    let mut canvas = Canvas::new(600, 600);
    draw_line(
        &mut canvas,
        CanvasPoint::new(-200, -100),
        CanvasPoint::new(240, 120),
        Color::new(255, 0, 0),
    );
    draw_line(
        &mut canvas,
        CanvasPoint::new(-50, -200),
        CanvasPoint::new(60, 240),
        Color::new(255, 0, 0),
    );
    canvas.save("image.png").unwrap();
}
