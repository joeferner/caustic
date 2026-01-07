use std::{
    any::Any,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use caustic_core::{
    Image,
    image::{ImageError, ImageImage},
};

use crate::resource_resolver::CodeResource;

#[derive(Debug)]
pub struct FileCodeResource {
    filename: PathBuf,
    code: String,
}

impl FileCodeResource {
    pub fn new(filename: &Path) -> std::io::Result<Self> {
        let code = fs::read_to_string(filename)?;
        Ok(Self {
            filename: filename.to_owned(),
            code,
        })
    }
}

impl CodeResource for FileCodeResource {
    fn get_code(&self) -> &str {
        &self.code
    }

    fn equals(&self, other: &dyn CodeResource) -> bool {
        self.get_code() == other.get_code()
            && other
                .as_any()
                .downcast_ref::<FileCodeResource>()
                .is_some_and(|other| self.filename == other.filename)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_image(&self, filename: &str) -> Result<Arc<dyn Image>, ImageError> {
        let dir = self.filename.parent().ok_or(ImageError::Other(format!(
            "source file \"{:?}\" has no parent",
            self.filename
        )))?;
        let image_filename = dir.join(filename);
        ImageImage::load_file(image_filename)
    }
}
