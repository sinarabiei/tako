pub mod camera;
pub mod canvas;
pub mod color;
pub mod light;
pub mod rasterizer;
pub mod scene;
pub mod sphere;

pub mod prelude {
    pub use crate::camera::Camera;
    pub use crate::canvas::Canvas;
    pub use crate::color::Color;
    pub use crate::light::Light;
    pub use crate::rasterizer::{draw_filled_triangle, draw_line, draw_wireframe_triangle};
    pub use crate::scene::Scene;
    pub use crate::sphere::Sphere;
}
