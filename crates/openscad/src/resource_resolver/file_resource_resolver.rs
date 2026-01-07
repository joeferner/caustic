use std::{path::Path, sync::Arc};

use crate::resource_resolver::{CodeResource, FileCodeResource, ResourceResolver};

pub struct FileResourceResolver {
    main: Arc<dyn CodeResource>,
}

impl FileResourceResolver {
    pub fn new(filename: &Path) -> std::io::Result<Self> {
        let main = Arc::new(FileCodeResource::new(filename)?);
        Ok(Self { main })
    }
}

impl ResourceResolver for FileResourceResolver {
    fn get_main(&self) -> Arc<dyn CodeResource> {
        self.main.clone()
    }
}
