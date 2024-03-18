use glam::Vec3;

pub enum Light {
    Ambient(AmbientLight),
    Point(PointLight),
    Directional(DirectionalLight),
}

impl Light {
    pub fn ambient(intensity: f32) -> Self {
        Light::Ambient(AmbientLight::new(intensity))
    }

    pub fn point(intensity: f32, position: Vec3) -> Self {
        Light::Point(PointLight::new(intensity, position))
    }

    pub fn directional(intensity: f32, direction: Vec3) -> Self {
        Light::Directional(DirectionalLight::new(intensity, direction))
    }
}

pub struct AmbientLight {
    pub intensity: f32,
}

impl AmbientLight {
    pub fn new(intensity: f32) -> Self {
        Self { intensity }
    }
}

pub struct PointLight {
    pub intensity: f32,
    pub position: Vec3,
}

impl PointLight {
    pub fn new(intensity: f32, position: Vec3) -> Self {
        Self {
            intensity,
            position,
        }
    }

    pub fn diffuse(&self, sphere_pt: Vec3, normal: Vec3) -> f32 {
        let light_vec = self.position - sphere_pt;
        let normal_dot_light = normal.dot(light_vec);
        if normal_dot_light > 0.0 {
            self.intensity * normal_dot_light / (normal.length() * light_vec.length())
        } else {
            0.0
        }
    }

    pub fn specular(
        &self,
        sphere_pt: Vec3,
        normal: Vec3,
        view_vec: Vec3,
        specular: Option<f32>,
    ) -> f32 {
        if let Some(s) = specular {
            let light_vec = self.position - sphere_pt;
            let reflect = 2.0 * normal * normal.dot(light_vec) - light_vec;
            let reflect_dot_view = reflect.dot(view_vec);
            if reflect_dot_view > 0.0 {
                self.intensity * (reflect_dot_view / (reflect.length() * view_vec.length())).powf(s)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}

pub struct DirectionalLight {
    pub intensity: f32,
    pub direction: Vec3,
}

impl DirectionalLight {
    pub fn new(intensity: f32, direction: Vec3) -> Self {
        Self {
            intensity,
            direction,
        }
    }

    pub fn diffuse(&self, normal: Vec3) -> f32 {
        let light_vec = self.direction;
        let normal_dot_light = normal.dot(light_vec);
        if normal_dot_light > 0.0 {
            self.intensity * normal_dot_light / (normal.length() * light_vec.length())
        } else {
            0.0
        }
    }

    pub fn specular(&self, normal: Vec3, view_vec: Vec3, specular: Option<f32>) -> f32 {
        if let Some(s) = specular {
            let light_vec = self.direction;
            let reflect = 2.0 * normal * normal.dot(light_vec) - light_vec;
            let reflect_dot_view = reflect.dot(view_vec);
            if reflect_dot_view > 0.0 {
                self.intensity * (reflect_dot_view / (reflect.length() * view_vec.length())).powf(s)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}
