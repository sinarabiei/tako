use glam::f32::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub width: f32,
    pub height: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            width: 1.0,
            height: 1.0,
        }
    }
}
