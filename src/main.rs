use image;
use indicatif::{ProgressBar, ProgressStyle};

use crate::color::Color;

pub mod color;
pub mod vector;

fn main() {
    let width: u32 = 400;
    let height: u32 = 200;

    let pb = ProgressBar::new(height as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap(),
    );

    let mut img: image::ImageBuffer<
        image::Rgb<u8>,
        Vec<<image::Rgb<u8> as image::Pixel>::Subpixel>,
    > = image::ImageBuffer::new(width, height);

    for y in 0..img.height() {
        for x in 0..img.width() {
            if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                *pixel = Color {
                    r: x as f64 / width as f64,
                    g: 0.0,
                    b: y as f64 / height as f64,
                }
                .into();
            }
        }
        pb.inc(1);
    }

    if let Some(pixel) = img.get_pixel_mut_checked(10, 10) {
        *pixel = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
        .into();
    }

    img.save("target/out.png").unwrap();
    pb.finish_with_message("Done!");
}
