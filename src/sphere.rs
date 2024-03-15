use crate::{color::Color, prelude::Camera};
use glam::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, color: Color) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }

    /// Evaluate `t` values for intersection between `camera` to `point`
    /// ray and the sphere.
    pub fn intersect(&self, camera: &Camera, point: Vec3) -> (f32, f32) {
        let r = self.radius;
        let center_to_camera = camera.position - self.center;

        let a = point.dot(point);
        let b = 2.0 * center_to_camera.dot(point);
        let c = center_to_camera.dot(center_to_camera) - r * r;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return (f32::INFINITY, f32::INFINITY);
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
        (t1, t2)
    }
}
