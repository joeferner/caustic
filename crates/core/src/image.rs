use std::{fmt::Debug, sync::Arc};

use crate::Color;

#[derive(Debug)]
pub enum ImageLoaderError {
    Io(String),
    Decode(String),
}

pub trait Image: Send + Sync + Debug {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn get_pixel(&self, x: u32, y: u32) -> Option<Color>;
}

pub trait ImageLoader: Send + Sync {
    fn load_image(&self, name: &str) -> Result<Arc<dyn Image>, ImageLoaderError>;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn image_loader_new() -> Arc<dyn ImageLoader> {
    use crate::image::image_crate::ImageImageLoader;

    Arc::new(ImageImageLoader::new())
}

#[cfg(not(target_arch = "wasm32"))]
pub mod image_crate {
    use std::sync::Arc;

    use image::{DynamicImage, GenericImageView, ImageReader, Pixel};

    use crate::{Color, Image, ImageLoader, image::ImageLoaderError};

    pub struct ImageImageLoader {}

    impl ImageImageLoader {
        pub fn new() -> Self {
            ImageImageLoader {}
        }
    }

    impl ImageLoader for ImageImageLoader {
        fn load_image(&self, name: &str) -> Result<Arc<dyn Image>, ImageLoaderError> {
            match ImageReader::open(name) {
                Ok(image) => match image.decode() {
                    Ok(image) => Ok(Arc::new(ImageImage { image })),
                    Err(err) => Err(ImageLoaderError::Decode(format!(
                        "Failed to decode image {name}: {err}"
                    ))),
                },
                Err(err) => Err(ImageLoaderError::Io(format!(
                    "Failed to load image {name}: {err}"
                ))),
            }
        }
    }

    impl Default for ImageImageLoader {
        fn default() -> Self {
            Self::new()
        }
    }

    #[derive(Debug)]
    pub struct ImageImage {
        image: DynamicImage,
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

#[cfg(target_arch = "wasm32")]
pub fn image_loader_new() -> Arc<dyn ImageLoader> {
    use crate::image::canvas::HtmlImageLoader;

    Arc::new(HtmlImageLoader::new())
}

#[cfg(target_arch = "wasm32")]
pub mod canvas {
    pub struct HtmlImageLoader {}
}
