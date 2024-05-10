use std::f32::consts::PI;

use glam::{Mat4, Vec3};
use tako::prelude::*;
use tako::rasterizer::Camera;

fn main() {
    let mut canvas = Canvas::new(600, 600);
    let camera = Camera::new(
        Vec3::new(-3.0, 1.0, 2.0),
        Mat4::from_rotation_y(-30.0 * PI / 180.0),
    );

    let vertices = vec![
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, -1.0, -1.0),
    ];
    let red = Color::new(255, 0, 0);
    let green = Color::new(0, 255, 0);
    let blue = Color::new(0, 0, 255);
    let yellow = Color::new(255, 255, 0);
    let purple = Color::new(255, 0, 255);
    let cyan = Color::new(0, 255, 255);
    let triangles = vec![
        Triangle::new([0, 1, 2], red),
        Triangle::new([0, 2, 3], red),
        Triangle::new([4, 0, 3], green),
        Triangle::new([4, 3, 7], green),
        Triangle::new([5, 4, 7], blue),
        Triangle::new([5, 7, 6], blue),
        Triangle::new([1, 5, 6], yellow),
        Triangle::new([1, 6, 2], yellow),
        Triangle::new([4, 5, 1], purple),
        Triangle::new([4, 1, 0], purple),
        Triangle::new([2, 6, 7], cyan),
        Triangle::new([2, 6, 7], cyan),
    ];
    let model = Model::new(vertices, triangles);
    let scene = vec![
        Instance::new(
            &model,
            Vec3::new(0.75, 0.75, 0.75),
            Mat4::IDENTITY,
            Vec3::new(-1.5, 0.0, 7.0),
        ),
        Instance::new(
            &model,
            Vec3::new(1.0, 1.0, 1.0),
            Mat4::from_rotation_y(195.0 * PI / 180.0),
            Vec3::new(1.25, 2.5, 7.5),
        ),
    ];

    render_scene(&mut canvas, &camera, &scene);

    canvas.save("image.png").unwrap();
}
