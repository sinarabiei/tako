use crate::prelude::*;
use glam::Vec3;
use image::{ImageResult, RgbImage};
use std::ops::{Div, Mul, RangeInclusive};

pub struct Canvas {
    width: u32,
    height: u32,
    image: RgbImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            image: RgbImage::new(width, height),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Converts canvas to screen coordinate automatically.
    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        let (screen_x, screen_y) = self.to_screen(x, y);
        self.image.put_pixel(screen_x, screen_y, color.into());
    }

    /// Saves the canvas to a file at the path specified.
    ///
    /// The image format is derived from the file extension.
    pub fn save(&self, path: &str) -> ImageResult<()> {
        self.image.save(path)
    }

    /// Range of valid x indices.
    fn x_range(&self) -> RangeInclusive<i32> {
        let half_width = (self.width / 2) as i32;
        let min = -((self.width / 2) as i32);
        let max = if self.width % 2 == 0 {
            half_width - 1
        } else {
            half_width
        };
        min..=max
    }

    /// Range of valid y indices.
    fn y_range(&self) -> RangeInclusive<i32> {
        let half_height = (self.height / 2) as i32;
        let min = if self.height % 2 == 0 {
            -half_height + 1
        } else {
            -half_height
        };
        let max = (self.height / 2) as i32;
        min..=max
    }

    /// Converts canvas coordinate to screen coordinate.
    fn to_screen(&self, x: i32, y: i32) -> (u32, u32) {
        let half_width = (self.width / 2) as i32;
        let half_height = (self.height / 2) as i32;
        ((half_width + x) as u32, (half_height - y) as u32)
    }

    /// Converts canvas coordinate, 2D, to space coordinate
    /// of the point on the projection plain, 3D.
    ///
    /// For the time being view's z coordinate is 1.0, distance between
    /// `Camera.position` and the projection plain.
    pub fn to_view(&self, camera: &Camera, x: i32, y: i32) -> Vec3 {
        Vec3::new(
            (x as f32).mul(camera.view_width().div(self.width as f32)),
            (y as f32).mul(camera.view_height().div(self.height as f32)),
            1.0,
        )
    }

    pub fn render(&mut self, scene: &Scene, camera: &Camera) {
        for x in self.x_range() {
            for y in self.y_range() {
                let p = self.to_view(camera, x, y);
                let color = scene.trace(camera, p, 1.0, f32::INFINITY);
                self.put_pixel(x, y, color);
            }
        }
    }
}