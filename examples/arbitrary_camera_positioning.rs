use glam::{Mat3, Vec3};
use std::f32::consts::PI;
use tako::prelude::*;

fn main() {
    let camera = Camera::new(
        Vec3::new(-1.0, 1.0, -3.0),
        Mat3::from_rotation_x(PI / 19.0)
            * Mat3::from_rotation_y(PI / 29.0)
            * Mat3::from_rotation_z(PI / 13.0),
    );
    let scene = Scene::new(
        vec![
            Sphere::new(
                Vec3::new(0.0, -1.0, 3.0),
                1.0,
                Color::new(255, 0, 0),
                Some(500),
                Some(0.2),
            ),
            Sphere::new(
                Vec3::new(2.0, 0.0, 4.0),
                1.0,
                Color::new(0, 0, 255),
                Some(500),
                Some(0.3),
            ),
            Sphere::new(
                Vec3::new(-2.0, 0.0, 4.0),
                1.0,
                Color::new(0, 255, 0),
                Some(10),
                Some(0.4),
            ),
            Sphere::new(
                Vec3::new(0.0, -5001.0, 0.0),
                5000.0,
                Color::new(255, 255, 0),
                Some(1000),
                Some(0.5),
            ),
        ],
        vec![
            Light::ambient(0.2),
            Light::point(0.6, Vec3::new(2.0, 1.0, 0.0)),
            Light::directional(0.2, Vec3::new(1.0, 4.0, 4.0)),
        ],
    );
    let mut canvas = Canvas::new(600, 600);
    canvas.render(&scene, &camera);
    canvas.save("image.png").unwrap();
    println!("   Raytracing finished.");
    println!("   See the result in \"image.png\" file.")
}
