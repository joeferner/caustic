use std::ops::{Add, Mul};

use image;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, v: Color) -> Color {
        Color {
            r: v.r * self,
            g: v.g * self,
            b: v.b * self,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Into<image::Rgb<u8>> for Color {
    fn into(self) -> image::Rgb<u8> {
        let r = (self.r * 255.999) as u8;
        let g = (self.g * 255.999) as u8;
        let b = (self.b * 255.999) as u8;
        image::Rgb([r, g, b])
    }
}
