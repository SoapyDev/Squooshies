use std::path::PathBuf;
use std::path::Path;
use std::time::SystemTime;
use image::DynamicImage;
use crate::error::TransformationError;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Picture {
    pub path: PathBuf,
    pub name: FileName,
    pub is_selected: bool,
    pub is_in_process: bool,
    pub is_processed: bool,
    pub metadata: Metadata, 
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct FileName {
    pub source_name: String,
    pub prefix: String,
    pub suffix: String,
}


#[derive(Clone, PartialEq, Debug, Default)]
pub struct Metadata {
    pub weight: u64,
    pub created: Option<SystemTime>,
    pub modified: Option<SystemTime>,
    pub accessed: Option<SystemTime>,
}

impl Picture {
    pub fn new(path: PathBuf) -> Self{
        let file = path.file_stem().unwrap_or_default().to_str().unwrap_or_default();
        let name = FileName::new(file);
        let metadata = Metadata::new(&path);
        Self { path, name, metadata,  is_selected:  true, is_in_process: false, is_processed: false }
    }
    pub fn get_name(& self) -> String {
        self.name.source_name.to_string()
    }

    pub fn get_weight(&self) -> String {
        let weight = self.metadata.weight;
        if weight > 1048576 {
            format!("{:.2} MB", weight as f32 / 1024.0 / 1024.0)
        } else if weight > 1024 {
            format!("{:.2} KB", weight as f32 / 1024.0)
        } else {
            format!("{} B", weight)
        }
    }
    
    pub fn get_path(&self) -> &str{
        self.path
            .to_str()
            .unwrap_or_default()
            .strip_prefix("C:\\")
            .unwrap_or(self.path.to_str().unwrap_or_default())
    }


    pub(crate) fn load(&self) -> Result<DynamicImage, TransformationError> {
        image::open(self.path.clone()).map_err(|e| TransformationError::Image(e.to_string()))
    }
}

impl FileName{
    pub fn new(name: &str) -> Self{
        Self{source_name : name.to_string(), prefix: String::new(), suffix: String::new()}
    }
    
    pub fn build(&self, path: &PathBuf) -> PathBuf {
        path.to_owned()
            .join(&self.prefix)
            .join(&self.source_name)
            .join(&self.suffix)
    }
}
#[cfg(target_os = "linux")]
use std::os::unix::prelude::MetadataExt;
impl Metadata{

    #[cfg(target_os = "linux")]
    pub fn new(path: &Path) -> Self {
        let metadata = path.metadata().unwrap();
        Self {
            weight: metadata.size(),
            created: metadata.created().ok(),
            modified: metadata.modified().ok(),
            accessed: metadata.accessed().ok(),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn new(path: &Path) -> Self {
        let metadata = path.metadata().unwrap();
        Self {
            weight: metadata.len(),
            created: metadata.created().ok(),
            modified: metadata.modified().ok(),
            accessed: metadata.accessed().ok(),
        }
    }
}

