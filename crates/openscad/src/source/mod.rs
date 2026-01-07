#[cfg(not(target_arch = "wasm32"))]
mod file_source;
mod string_source;

use caustic_core::{Image, image::ImageError};
#[cfg(not(target_arch = "wasm32"))]
pub use file_source::FileSource;
use std::{any::Any, fmt::Debug, sync::Arc};
pub use string_source::StringSource;

pub trait Source: Debug {
    fn get_code(&self) -> &str;
    fn get_image(&self, filename: &str) -> Result<Arc<dyn Image>, ImageError>;
    fn as_any(&self) -> &dyn Any;

    fn equals(&self, other: &dyn Source) -> bool {
        self.get_code() == other.get_code()
    }
}
