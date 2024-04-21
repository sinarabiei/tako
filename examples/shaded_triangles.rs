use glam::Vec2;
use tako::prelude::*;

fn main() {
    let mut canvas = Canvas::new(600, 600);
    draw_shaded_triangle(
        &mut canvas,
        Vertex::new(Vec2::new(-200.0, -250.0), 0.5),
        Vertex::new(Vec2::new(200.0, 50.0), 0.0),
        Vertex::new(Vec2::new(20.0, 250.0), 1.0),
        Color::new(0, 255, 0),
    );
    canvas.save("image.png").unwrap();
}
