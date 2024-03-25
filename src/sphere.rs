use crate::color::Color;
use glam::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color,
    pub specular: Option<i32>,
    pub reflective: Option<f32>,
}

impl Sphere {
    pub fn new(
        center: Vec3,
        radius: f32,
        color: Color,
        specular: Option<i32>,
        reflective: Option<f32>,
    ) -> Self {
        Self {
            center,
            radius,
            color,
            specular,
            reflective,
        }
    }

    /// Evaluate `t` values for intersection between `camera` to `point`
    /// ray and the sphere.
    pub fn intersect(&self, origin: Vec3, direction: Vec3) -> (f32, f32) {
        let r = self.radius;
        let center_to_origin = origin - self.center;

        let a = direction.dot(direction);
        let b = 2.0 * center_to_origin.dot(direction);
        let c = center_to_origin.dot(center_to_origin) - r * r;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return (f32::INFINITY, f32::INFINITY);
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
        (t1, t2)
    }
}
