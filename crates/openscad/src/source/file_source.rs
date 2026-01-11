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

use crate::source::Source;

#[derive(Debug)]
pub struct FileSource {
    filename: String,
    filename_path: PathBuf,
    code: String,
}

impl FileSource {
    pub fn new(filename_path: &Path) -> std::io::Result<Self> {
        let filename = filename_path.to_string_lossy().to_string();
        let code = fs::read_to_string(filename_path)?;
        Ok(Self {
            filename,
            filename_path: filename_path.to_owned(),
            code,
        })
    }
}

impl Source for FileSource {
    fn get_code(&self) -> &str {
        &self.code
    }

    fn equals(&self, other: &dyn Source) -> bool {
        self.get_code() == other.get_code()
            && other
                .as_any()
                .downcast_ref::<FileSource>()
                .is_some_and(|other| self.filename_path == other.filename_path)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_image(&self, filename: &str) -> Result<Arc<dyn Image>, ImageError> {
        let dir = self
            .filename_path
            .parent()
            .ok_or(ImageError::Other(format!(
                "source file \"{:?}\" has no parent",
                self.filename_path
            )))?;
        let image_filename = dir.join(filename);
        ImageImage::load_file(image_filename)
    }

    fn get_filename(&self) -> &str {
        &self.filename
    }
}
