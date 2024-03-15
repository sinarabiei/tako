use crate::prelude::*;
use glam::Vec3;

pub struct Scene(Vec<Sphere>);

impl Scene {
    pub fn new(scene: Vec<Sphere>) -> Self {
        Self(scene)
    }

    /// Traces the ray from `camera` through the `point` on the projection plain.
    pub fn trace(&self, camera: &Camera, point: Vec3, t_min: f32, t_max: f32) -> Color {
        let mut closest_t = f32::INFINITY;
        let mut closest_sphere = None;

        for sphere in self.0.iter() {
            let (t1, t2) = sphere.intersect(&camera, point);
            if (t1 >= t_min || t1 <= t_max) && t1 < closest_t {
                closest_t = t1;
                closest_sphere = Some(sphere);
            }
            if (t2 >= t_min || t2 < t_max) && t2 < closest_t {
                closest_t = t2;
                closest_sphere = Some(sphere);
            }
        }
        match closest_sphere {
            Some(sphere) => sphere.color,
            None => Color::BACKGROUND_COLOR,
        }
    }
}
