use crate::{camera::Camera, color::Color, light::Light, sphere::Sphere};
use glam::Vec3;

pub struct Scene {
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new(spheres: Vec<Sphere>, lights: Vec<Light>) -> Self {
        Self { spheres, lights }
    }

    /// Traces the ray from `camera` through the `point` on the projection plain.
    pub fn trace(&self, camera: &Camera, view_pt: Vec3, t_min: f32, t_max: f32) -> Color {
        let mut closest_t = f32::INFINITY;
        let mut closest_sphere = None;

        for sphere in self.spheres.iter() {
            let (t1, t2) = sphere.intersect(&camera, view_pt);
            if (t1 >= t_min && t1 <= t_max) && t1 < closest_t {
                closest_t = t1;
                closest_sphere = Some(sphere);
            }
            if (t2 >= t_min && t2 < t_max) && t2 < closest_t {
                closest_t = t2;
                closest_sphere = Some(sphere);
            }
        }

        if let Some(sphere) = closest_sphere {
            let camera_vec = view_pt - camera.position;
            let sphere_pt = camera.position + closest_t * (camera_vec);
            let normal = (sphere_pt - sphere.center).normalize();
            sphere.color * self.light(sphere_pt, normal, -camera_vec, sphere.specular)
        } else {
            Color::BACKGROUND_COLOR
        }
    }

    fn light(&self, sphere_pt: Vec3, normal: Vec3, view_vec: Vec3, specular: Option<f32>) -> f32 {
        let mut intensity = 0.0;
        for light in self.lights.iter() {
            match light {
                Light::Ambient(ambient) => intensity += ambient.intensity,
                Light::Point(point) => {
                    intensity += point.diffuse(sphere_pt, normal);
                    intensity += point.specular(sphere_pt, normal, view_vec, specular);
                }
                Light::Directional(directional) => {
                    intensity += directional.diffuse(normal);
                    intensity += directional.specular(normal, view_vec, specular);
                }
            }
        }
        intensity
    }
}
