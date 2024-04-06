use crate::{camera::Camera, color::Color, scene::Scene};
use glam::{Mat3, Vec3};
use image::{ImageResult, RgbImage};
use std::{
    ops::{Div, Mul},
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

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

    /// Converts canvas to screen coordinate automatically.
    fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        let (screen_x, screen_y) = self.to_screen(x, y);
        self.image.put_pixel(screen_x, screen_y, color.into());
    }

    /// Saves the canvas to a file at the path specified.
    ///
    /// The image format is derived from the file extension.
    pub fn save(&self, path: &str) -> ImageResult<()> {
        self.image.save(path)
    }

    fn x_min(&self) -> i32 {
        -((self.width / 2) as i32)
    }

    /// Exclusive
    fn x_max(&self) -> i32 {
        if self.width % 2 == 0 {
            (self.width / 2) as i32
        } else {
            (self.width / 2) as i32 + 1
        }
    }

    fn y_min(&self) -> i32 {
        if self.height % 2 == 0 {
            -((self.height / 2) as i32) + 1
        } else {
            -((self.height / 2) as i32)
        }
    }

    /// Exclusive
    fn y_max(&self) -> i32 {
        (self.height / 2) as i32 + 1
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
    fn to_view(
        canvas_width: u32,
        canvas_height: u32,
        view_width: f32,
        view_height: f32,
        x: i32,
        y: i32,
    ) -> Vec3 {
        Vec3::new(
            (x as f32).mul(view_width.div(canvas_width as f32)),
            (y as f32).mul(view_height.div(canvas_height as f32)),
            1.0,
        )
    }

    pub fn render(&mut self, scene: &Scene, camera: &Camera) {
        for x in self.x_min()..self.x_max() {
            for y in self.y_min()..self.y_max() {
                let direction = camera.rotation
                    * Canvas::to_view(self.width, self.height, camera.width, camera.height, x, y);
                let color = scene.trace(camera.position, direction, 1.0, f32::INFINITY, 3);
                self.put_pixel(x, y, color);
            }
        }
    }

    pub fn render_parallel(&mut self, scene: Arc<Scene>, camera: &Camera) {
        let (sender, reciever) = mpsc::channel();
        let mut handles = Vec::new();
        let thread_count = 5;
        for thread in 0..thread_count {
            let sender = sender.clone();
            let scene = Arc::clone(&scene);
            let canvas_width = self.width;
            let canvas_height = self.height;
            let x_min = self.x_min();
            let x_max = self.x_max();
            let offset = (x_max - x_min).abs() / thread_count;
            let x_start = x_min + thread * offset;
            let x_end = if thread < (thread_count - 1) {
                x_start + offset
            } else {
                x_max
            };
            let y_min = self.y_min();
            let y_max = self.y_max();
            let camera_rotation = camera.rotation;
            let view_width = camera.width;
            let view_height = camera.height;
            let camera_position = camera.position;
            handles.push(thread::spawn(move || {
                Canvas::render_section(
                    canvas_width,
                    canvas_height,
                    x_start,
                    x_end,
                    y_min,
                    y_max,
                    scene,
                    view_width,
                    view_height,
                    camera_position,
                    camera_rotation,
                    sender,
                );
            }))
        }
        drop(sender);
        let reciever = reciever;
        for (x, y, color) in reciever {
            self.put_pixel(x, y, color);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_section(
        canvas_width: u32,
        canvas_height: u32,
        x_min: i32,
        x_max: i32,
        y_min: i32,
        y_max: i32,
        scene: Arc<Scene>,
        view_width: f32,
        view_height: f32,
        camera_position: Vec3,
        camera_rotation: Mat3,
        sender: Sender<(i32, i32, Color)>,
    ) {
        for x in x_min..x_max {
            for y in y_min..y_max {
                let direction = camera_rotation
                    * Canvas::to_view(canvas_width, canvas_height, view_width, view_height, x, y);
                let color = scene.trace(camera_position, direction, 1.0, f32::INFINITY, 3);
                sender.send((x, y, color)).unwrap();
            }
        }
    }
}
