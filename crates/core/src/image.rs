use std::fmt::Debug;
use std::sync::Arc;

use crate::Color;

#[derive(Debug)]
pub enum ImageError {
    Io(String),
    Decode(String),
    Other(String),
}

pub trait Image: Send + Sync + Debug {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn get_pixel(&self, x: u32, y: u32) -> Option<Color>;
}

#[cfg(not(target_arch = "wasm32"))]
pub use image_crate::ImageImage;

#[cfg(not(target_arch = "wasm32"))]
pub mod image_crate {
    use std::sync::Arc;

    use image::{DynamicImage, GenericImageView, ImageReader, Pixel};

    use crate::{Color, Image, image::ImageError, utils::to_absolute};

    #[derive(Debug)]
    pub struct ImageImage {
        image: DynamicImage,
    }

    impl ImageImage {
        pub fn load_file(name: &str) -> Result<Arc<dyn Image>, ImageError> {
            let absolute_path = to_absolute(name).map_err(|err| {
                ImageError::Io(format!(
                    "could not convert name \"{name}\" to absolute path: {err:?}"
                ))
            })?;

            match ImageReader::open(&absolute_path) {
                Ok(image) => match image.decode() {
                    Ok(image) => Ok(Arc::new(ImageImage { image })),
                    Err(err) => Err(ImageError::Decode(format!(
                        "Failed to decode image {absolute_path:?}: {err}"
                    ))),
                },
                Err(err) => Err(ImageError::Io(format!(
                    "Failed to load image {absolute_path:?}: {err}"
                ))),
            }
        }
    }

    impl Image for ImageImage {
        fn width(&self) -> u32 {
            self.image.width()
        }

        fn height(&self) -> u32 {
            self.image.height()
        }

        fn get_pixel(&self, x: u32, y: u32) -> Option<crate::Color> {
            if !self.image.in_bounds(x, y) {
                return None;
            }
            let p = self.image.get_pixel(x, y).to_rgb();
            let r = p.0[0] as f64 / 255.0;
            let g = p.0[1] as f64 / 255.0;
            let b = p.0[2] as f64 / 255.0;
            Some(Color::new(r, g, b))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_image(filename: &str) -> Result<Arc<dyn Image>, ImageError> {
    ImageImage::load_file(filename)
}

#[cfg(target_arch = "wasm32")]
pub use html::HtmlImage;

#[cfg(target_arch = "wasm32")]
pub mod html {
    use crate::{Color, Image, image::ImageError};
    use std::sync::Arc;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct ImageData {
        width: u32,
        height: u32,
        pixels: Vec<u8>,
    }

    #[wasm_bindgen]
    impl ImageData {
        #[wasm_bindgen(constructor)]
        pub fn new(width: u32, height: u32, pixels: Vec<u8>) -> ImageData {
            ImageData {
                width,
                height,
                pixels,
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        fn load_image(name: &str) -> ImageData;
    }

    #[derive(Debug)]
    pub struct HtmlImage {
        width: u32,
        height: u32,
        pixels: Vec<Color>,
    }

    impl HtmlImage {
        pub fn load_url(name: &str) -> Result<Arc<dyn Image>, ImageError> {
            let img_data = load_image(name);
            // TODO verify image data meets width*height*3
            let width = img_data.width;
            let height = img_data.height;
            let pixels = img_data
                .pixels
                .chunks_exact(3)
                .map(|chunk| Color {
                    r: chunk[0] as f64 / 255.0,
                    g: chunk[1] as f64 / 255.0,
                    b: chunk[2] as f64 / 255.0,
                })
                .collect();

            let img = HtmlImage {
                width,
                height,
                pixels,
            };

            Ok(Arc::new(img))
        }
    }

    impl Image for HtmlImage {
        fn width(&self) -> u32 {
            self.width
        }

        fn height(&self) -> u32 {
            self.height
        }

        fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
            // Bounds check
            if x >= self.width || y >= self.height {
                return None;
            }

            // Calculate index in linear array
            let index = (y * self.width + x) as usize;
            self.pixels.get(index).copied()
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn load_image(url: &str) -> Result<Arc<dyn Image>, ImageError> {
    HtmlImage::load_url(url)
}
