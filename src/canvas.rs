use crate::color::Color;

use image::{ImageResult, RgbImage};

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

    pub fn min_x(&self) -> i32 {
        -((self.width / 2) as i32)
    }

    pub fn max_x(&self) -> i32 {
        let half_width = (self.width / 2) as i32;
        if self.width % 2 == 0 {
            half_width - 1
        } else {
            half_width
        }
    }

    pub fn min_y(&self) -> i32 {
        let half_height = (self.width / 2) as i32;
        if self.height % 2 == 0 {
            -half_height + 1
        } else {
            -half_height
        }
    }

    pub fn max_y(&self) -> i32 {
        (self.width / 2) as i32
    }

    /// Converts canvas coordinate to screen coordinate.
    fn to_screen(&self, x: i32, y: i32) -> (u32, u32) {
        let half_width = (self.width / 2) as i32;
        let half_height = (self.height / 2) as i32;
        ((half_width + x) as u32, (half_height - y) as u32)
    }
}
