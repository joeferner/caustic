#[cfg(not(target_arch = "wasm32"))]
mod file_code_resource;
#[cfg(not(target_arch = "wasm32"))]
mod file_resource_resolver;
mod string_code_resource;

use caustic_core::{Image, image::ImageError};
#[cfg(not(target_arch = "wasm32"))]
pub use file_code_resource::FileCodeResource;
#[cfg(not(target_arch = "wasm32"))]
pub use file_resource_resolver::FileResourceResolver;
use std::{any::Any, fmt::Debug, sync::Arc};
pub use string_code_resource::StringCodeResource;

pub trait ResourceResolver {
    fn get_main(&self) -> Arc<dyn CodeResource>;
}

pub trait CodeResource: Debug {
    fn get_code(&self) -> &str;
    fn get_image(&self, filename: &str) -> Result<Arc<dyn Image>, ImageError>;
    fn as_any(&self) -> &dyn Any;

    fn equals(&self, other: &dyn CodeResource) -> bool {
        self.get_code() == other.get_code()
    }
}
