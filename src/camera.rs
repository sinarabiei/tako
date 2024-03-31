use glam::{f32::Vec3, Mat3};

pub struct Camera {
    pub position: Vec3,
    pub width: f32,
    pub height: f32,
    pub rotation: Mat3,
}

impl Camera {
    pub fn new(position: Vec3, rotation: Mat3) -> Self {
        Self {
            position,
            width: 1.0,
            height: 1.0,
            rotation,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            width: 1.0,
            height: 1.0,
            rotation: Mat3::IDENTITY,
        }
    }
}
