use crate::{camera::Camera, color::Color};
use glam::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color,
    pub specular: Option<f32>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, color: Color, specular: Option<f32>) -> Self {
        Self {
            center,
            radius,
            color,
            specular,
        }
    }

    /// Evaluate `t` values for intersection between `camera` to `point`
    /// ray and the sphere.
    pub fn intersect(&self, camera: &Camera, view_pt: Vec3) -> (f32, f32) {
        let r = self.radius;
        let center_vec = camera.position - self.center;

        let a = view_pt.dot(view_pt);
        let b = 2.0 * center_vec.dot(view_pt);
        let c = center_vec.dot(center_vec) - r * r;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return (f32::INFINITY, f32::INFINITY);
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
        (t1, t2)
    }
}
