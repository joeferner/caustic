use std::{any::Any, sync::Arc};

use caustic_core::{Image, image::ImageError};

use crate::source::Source;

#[derive(Debug)]
pub struct StringSource {
    code: String,
}

impl StringSource {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_owned(),
        }
    }
}

impl Source for StringSource {
    fn get_code(&self) -> &str {
        &self.code
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_image(&self, filename: &str) -> Result<Arc<dyn Image>, ImageError> {
        todo!("get_image {filename}")
    }

    fn get_filename(&self) -> &str {
        "string"
    }
}
