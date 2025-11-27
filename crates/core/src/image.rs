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
    use std::sync::Arc;

    use image::{DynamicImage, GenericImageView, ImageReader, Pixel};

    use crate::{Color, Image, image::ImageError};

    #[derive(Debug)]
    pub struct ImageImage {
        image: DynamicImage,
    }

    impl ImageImage {
        pub fn load_file(name: &str) -> Result<Arc<dyn Image>, ImageError> {
            match ImageReader::open(name) {
                Ok(image) => match image.decode() {
                    Ok(image) => Ok(Arc::new(ImageImage { image })),
                    Err(err) => Err(ImageError::Decode(format!(
                        "Failed to decode image {name}: {err}"
                    ))),
                },
                Err(err) => Err(ImageError::Io(format!(
                    "Failed to load image {name}: {err}"
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

#[cfg(target_arch = "wasm32")]
pub use html::HtmlImage;

#[cfg(target_arch = "wasm32")]
pub mod html {
    use std::sync::Arc;

    use crate::{Image, image::ImageError};

    pub struct HtmlImage {}

    impl HtmlImage {
        pub fn load_url(url: &str) -> Result<Arc<dyn Image>, ImageError> {
            let window = web_sys::window().ok_or(ImageError::Other("No window".to_string()))?;
            let document = window
                .document()
                .ok_or(ImageError::Other("No document".to_string()))?;

            let img = document
                .get_element_by_id(url)
                .ok_or(ImageError::Io(format!("Could not load image: {url}")));
            todo!("{:?}", img);
        }
    }
}
