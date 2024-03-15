use glam::f32::Vec3;

struct View {
    width: f32,
    height: f32,
    // Center of the projection plane
    // position: Vec3,
    // Normal vector of the projection plane
    // rotation: Vec3,
}

pub struct Camera {
    pub position: Vec3,
    view: View,
}

impl Camera {
    pub fn view_width(&self) -> f32 {
        self.view.width
    }

    pub fn view_height(&self) -> f32 {
        self.view.height
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            view: View {
                width: 1.0,
                height: 1.0,
                // position: Vec3::new(0.0, 0.0, 1.0),
                // rotation: Vec3::new(0.0, 0.0, 1.0),
            },
        }
    }
}
