#[cfg(not(target_arch = "wasm32"))]
mod file_source;
mod string_source;

use caustic_core::{Image, image::ImageError, line_number_at_offset};
#[cfg(not(target_arch = "wasm32"))]
pub use file_source::FileSource;
use std::{any::Any, fmt::Debug, sync::Arc};
pub use string_source::StringSource;

pub trait Source: Debug {
    fn get_filename(&self) -> &str;
    fn get_code(&self) -> &str;
    fn get_image(&self, filename: &str) -> Result<Arc<dyn Image>, ImageError>;
    fn as_any(&self) -> &dyn Any;

    fn equals(&self, other: &dyn Source) -> bool {
        self.get_code() == other.get_code()
    }

    fn to_string(&self, start: usize, _end: usize) -> String {
        let line = line_number_at_offset(self.get_code(), start);
        format!("{}:{line}", self.get_filename())
    }
}

impl PartialEq for dyn Source {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
