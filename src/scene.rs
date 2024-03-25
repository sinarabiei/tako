use crate::{color::Color, light::Light, sphere::Sphere};
use glam::Vec3;

pub struct Scene {
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new(spheres: Vec<Sphere>, lights: Vec<Light>) -> Self {
        Self { spheres, lights }
    }

    pub fn trace(
        &self,
        origin: Vec3,
        direction: Vec3,
        t_min: f32,
        t_max: f32,
        recursion_depth: i32,
    ) -> Color {
        let (closest_sphere, closest_t) = self.intersect(origin, direction, t_min, t_max);
        match closest_sphere {
            Some(sphere) => {
                let point = origin + closest_t * direction;
                let normal = (point - sphere.center).normalize();
                let local_color =
                    sphere.color * self.light(point, normal, -direction, sphere.specular);
                if let Some(reflective) = sphere.reflective {
                    if recursion_depth <= 0 {
                        local_color
                    } else {
                        let reflect = Scene::reflect_ray(-direction, normal);
                        let reflected_color =
                            self.trace(point, reflect, 0.1, f32::INFINITY, recursion_depth - 1);
                        local_color * (1.0 - reflective) + reflected_color * reflective
                    }
                } else {
                    local_color
                }
            }
            None => Color::BACKGROUND_COLOR,
        }
    }

    fn light(&self, point: Vec3, normal: Vec3, view: Vec3, specular: Option<i32>) -> f32 {
        let mut intensity = 0.0;
        for light in self.lights.iter() {
            match light {
                Light::Ambient(ambient_light) => intensity += ambient_light.intensity,
                Light::Point(point_light) => {
                    let light = point_light.position - point;
                    let t_max = 1.0;
                    let (shadow_sphere, _shadow_t) = self.intersect(point, light, 0.001, t_max);
                    if shadow_sphere.is_none() {
                        if let Some(diffuse_value) = Scene::diffuse(light, normal) {
                            intensity += point_light.intensity * diffuse_value;
                        }
                        if let Some(specular_value) = Scene::specular(light, normal, view, specular)
                        {
                            intensity += point_light.intensity * specular_value;
                        }
                    }
                }
                Light::Directional(directional_light) => {
                    let light = directional_light.direction;
                    let t_max = f32::INFINITY;
                    let (shadow_sphere, _shadow_t) = self.intersect(point, light, 0.001, t_max);
                    if shadow_sphere.is_none() {
                        if let Some(diffuse_value) = Scene::diffuse(light, normal) {
                            intensity += directional_light.intensity * diffuse_value;
                        }
                        if let Some(specular_value) = Scene::specular(light, normal, view, specular)
                        {
                            intensity += directional_light.intensity * specular_value;
                        }
                    }
                }
            }
        }
        intensity
    }

    fn intersect(
        &self,
        origin: Vec3,
        direction: Vec3,
        t_min: f32,
        t_max: f32,
    ) -> (Option<&Sphere>, f32) {
        let mut closest_t = f32::INFINITY;
        let mut closest_sphere = None;
        for sphere in self.spheres.iter() {
            let (t1, t2) = sphere.intersect(origin, direction);
            if t1 >= t_min && t1 <= t_max && t1 < closest_t {
                closest_t = t1;
                closest_sphere = Some(sphere);
            }
            if t2 >= t_min && t2 <= t_max && t2 < closest_t {
                closest_t = t2;
                closest_sphere = Some(sphere);
            }
        }
        (closest_sphere, closest_t)
    }

    fn diffuse(light: Vec3, normal: Vec3) -> Option<f32> {
        let normal_dot_light = normal.dot(light);
        if normal_dot_light < 0.0 {
            None
        } else {
            Some(normal_dot_light / (normal.length() * light.length()))
        }
    }

    fn specular(light: Vec3, normal: Vec3, view: Vec3, specular: Option<i32>) -> Option<f32> {
        if let Some(exponent) = specular {
            let reflect = Scene::reflect_ray(light, normal);
            let reflect_dot_view = reflect.dot(view);
            if reflect_dot_view > 0.0 {
                Some((reflect_dot_view / (reflect.length() * view.length())).powi(exponent))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn reflect_ray(ray: Vec3, normal: Vec3) -> Vec3 {
        2.0 * normal * normal.dot(ray) - ray
    }
}
