use std::fmt::Debug;

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
    use std::{path::Path, sync::Arc};

    use image::{DynamicImage, GenericImageView, ImageReader, Pixel};

    use crate::{Color, Image, image::ImageError};

    #[derive(Debug)]
    pub struct ImageImage {
        image: DynamicImage,
    }

    impl ImageImage {
        pub fn load_file<P>(filename: P) -> Result<Arc<dyn Image>, ImageError>
        where
            P: AsRef<Path>,
        {
            match ImageReader::open(filename) {
                Ok(image) => match image.decode() {
                    Ok(image) => Ok(Arc::new(ImageImage { image })),
                    Err(err) => Err(ImageError::Decode(format!("Failed to decode image: {err}"))),
                },
                Err(err) => Err(ImageError::Io(format!("Failed to load image: {err}"))),
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
