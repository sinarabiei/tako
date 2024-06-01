use image::Rgb;
use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const BACKGROUND_COLOR: Color = Color {
        red: 0,
        green: 0,
        blue: 0,
    };

    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        Rgb([value.red, value.green, value.blue])
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    /// Clamps color channels, [0-255]
    fn mul(self, rhs: f32) -> Self::Output {
        let red = (self.red as f32).mul(rhs);
        let green = (self.green as f32).mul(rhs);
        let blue = (self.blue as f32).mul(rhs);
        Self {
            red: red.clamp(0.0, 255.0) as u8,
            green: green.clamp(0.0, 255.0) as u8,
            blue: blue.clamp(0.0, 255.0) as u8,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    /// Clamps color channels, [0-255]
    fn mul(self, rhs: Color) -> Self::Output {
        let red = self.mul(rhs.red as f32);
        let green = self.mul(rhs.green as f32);
        let blue = self.mul(rhs.blue as f32);
        Color {
            red: red.clamp(0.0, 0.255) as u8,
            green: green.clamp(0.0, 0.255) as u8,
            blue: blue.clamp(0.0, 0.255) as u8,
        }
    }
}

impl Add for Color {
    type Output = Self;

    /// Clamps color channels, [0-255]
    fn add(self, rhs: Self) -> Self::Output {
        let red = (self.red as i32).add(rhs.red as i32);
        let green = (self.green as i32).add(rhs.green as i32);
        let blue = (self.blue as i32).add(rhs.blue as i32);
        Self {
            red: red.clamp(0, 255) as u8,
            green: green.clamp(0, 255) as u8,
            blue: blue.clamp(0, 255) as u8,
        }
    }
}
