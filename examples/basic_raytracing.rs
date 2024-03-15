use glam::Vec3;
use tako::prelude::*;

fn main() {
    let camera = Camera::default();
    let scene = Scene::new(vec![
        Sphere::new(Vec3::new(0.0, -1.0, 3.0), 1.0, Color::new(255, 0, 0)),
        Sphere::new(Vec3::new(2.0, 0.0, 4.0), 1.0, Color::new(0, 0, 255)),
        Sphere::new(Vec3::new(-2.0, 0.0, 4.0), 1.0, Color::new(0, 255, 0)),
    ]);
    let mut canvas = Canvas::new(600, 600);

    canvas.render(&scene, &camera);

    canvas.save("image.png").unwrap();
    println!("   Raytracing finished.");
    println!("   See the result in \"image.png\" file.")
}
