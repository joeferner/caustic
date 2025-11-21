use image;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Into<image::Rgb<u8>> for Color {
    fn into(self) -> image::Rgb<u8> {
        let r = (self.r * 255.0) as u8;
        let g = (self.g * 255.0) as u8;
        let b = (self.b * 255.0) as u8;
        image::Rgb([r, g, b])
    }
}
