use glam::Vec3;
use tako::{prelude::*, rasterizer::project_vertex};

fn main() {
    let mut canvas = Canvas::new(600, 600);
    let camera = Camera::default();
    // The four "front" vertices
    let a_front = Vec3::new(-2.0, -0.5, 5.0);
    let b_front = Vec3::new(-2.0, 0.5, 5.0);
    let c_front = Vec3::new(-1.0, 0.5, 5.0);
    let d_front = Vec3::new(-1.0, -0.5, 5.0);
    // The four "back" vertices
    let a_back = Vec3::new(-2.0, -0.5, 6.0);
    let b_back = Vec3::new(-2.0, 0.5, 6.0);
    let c_back = Vec3::new(-1.0, 0.5, 6.0);
    let d_back = Vec3::new(-1.0, -0.5, 6.0);
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();
    // The front face
    draw_line(
        &mut canvas,
        project_vertex(canvas_width, canvas_height, &camera, a_front),
        project_vertex(canvas_width, canvas_height, &camera, b_front),
        Color::new(0, 0, 255),
    );
    draw_line(
        &mut canvas,
        project_vertex(canvas_width, canvas_height, &camera, b_front),
        project_vertex(canvas_width, canvas_height, &camera, c_front),
        Color::new(0, 0, 255),
    );
    draw_line(
        &mut canvas,
        project_vertex(canvas_width, canvas_height, &camera, c_front),
        project_vertex(canvas_width, canvas_height, &camera, d_front),
        Color::new(0, 0, 255),
    );
    draw_line(
        &mut canvas,
        project_vertex(canvas_width, canvas_height, &camera, d_front),
        project_vertex(canvas_width, canvas_height, &camera, a_front),
        Color::new(0, 0, 255),
    );
    // The back face
    draw_line(
        &mut canvas,
        project_vertex(canvas_width, canvas_height, &camera, a_back),
        project_vertex(canvas_width, canvas_height, &camera, b_back),
        Color::new(255, 0, 0),
    );
    draw_line(
        &mut canvas,
        project_vertex(canvas_width, canvas_height, &camera, b_back),
        project_vertex(canvas_width, canvas_height, &camera, c_back),
        Color::new(255, 0, 0),
    );
    draw_line(
        &mut canvas,
        project_vertex(canvas_width, canvas_height, &camera, c_back),
        project_vertex(canvas_width, canvas_height, &camera, d_back),
        Color::new(255, 0, 0),
    );
    draw_line(
        &mut canvas,
        project_vertex(canvas_width, canvas_height, &camera, d_back),
        project_vertex(canvas_width, canvas_height, &camera, a_back),
        Color::new(255, 0, 0),
    );
    // The front-to-back edges
    draw_line(
        &mut canvas,
        project_vertex(canvas_width, canvas_height, &camera, a_front),
        project_vertex(canvas_width, canvas_height, &camera, a_back),
        Color::new(0, 255, 0),
    );
    draw_line(
        &mut canvas,
        project_vertex(canvas_width, canvas_height, &camera, b_front),
        project_vertex(canvas_width, canvas_height, &camera, b_back),
        Color::new(0, 255, 0),
    );
    draw_line(
        &mut canvas,
        project_vertex(canvas_width, canvas_height, &camera, c_front),
        project_vertex(canvas_width, canvas_height, &camera, c_back),
        Color::new(0, 255, 0),
    );
    draw_line(
        &mut canvas,
        project_vertex(canvas_width, canvas_height, &camera, d_front),
        project_vertex(canvas_width, canvas_height, &camera, d_back),
        Color::new(0, 255, 0),
    );
    canvas.save("image.png").unwrap();
}
