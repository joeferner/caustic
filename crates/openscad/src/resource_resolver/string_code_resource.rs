use std::{any::Any, sync::Arc};

use caustic_core::{Image, image::ImageError};

use crate::resource_resolver::CodeResource;

#[derive(Debug)]
pub struct StringCodeResource {
    code: String,
}

impl StringCodeResource {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_owned(),
        }
    }
}

impl CodeResource for StringCodeResource {
    fn get_code(&self) -> &str {
        &self.code
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_image(&self, filename: &str) -> Result<Arc<dyn Image>, ImageError> {
        todo!("get_image {filename}")
    }
}
