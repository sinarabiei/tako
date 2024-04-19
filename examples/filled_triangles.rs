use glam::Vec2;
use tako::prelude::*;

fn main() {
    let mut canvas = Canvas::new(600, 600);
    draw_filled_triangle(
        &mut canvas,
        Vec2::new(-200.0, -250.0),
        Vec2::new(200.0, 50.0),
        Vec2::new(20.0, 250.0),
        Color::new(0, 255, 0),
    );
    draw_wireframe_triangle(
        &mut canvas,
        Vec2::new(-200.0, -250.0),
        Vec2::new(200.0, 50.0),
        Vec2::new(20.0, 250.0),
        Color::new(255, 0, 0),
    );
    canvas.save("image.png").unwrap();
}
