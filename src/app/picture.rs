use std::fs::File;
use std::io::BufReader;
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
    pub width: usize,
    pub height: usize,
    pub rotation: Option<u32>,
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
    pub fn get_name(&self) -> String {
        self.path.file_name().unwrap_or_default().to_str().unwrap_or_default().to_string()
    }

    pub fn get_size(&self) -> String{
        format!("{}px - {}px", self.metadata.width, self.metadata.height)
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
        image::open(self.path.to_str().unwrap_or_default()).map_err(|e| TransformationError::Image(e.to_string()))
    }
}

impl FileName{
    pub fn new(name: &str) -> Self{
        Self{source_name : name.to_string(), prefix: String::new(), suffix: String::new()}
    }
    
    pub fn build(&self, path: &Path) -> PathBuf {
        path.to_owned()
            .join(&self.prefix)
            .join(&self.source_name)
            .join(&self.suffix)
    }
}
#[cfg(target_os = "linux")]
use std::os::unix::prelude::MetadataExt;
use exif::{Exif, In, Tag};

impl Metadata{

    #[cfg(target_os = "linux")]
    pub fn new(path: &Path) -> Self {
        let metadata = path.metadata().unwrap();
        let (width, height) = get_image_size(path);
        Self {
            weight: metadata.size(),
            width,
            height,
            created: metadata.created().ok(),
            modified: metadata.modified().ok(),
            accessed: metadata.accessed().ok(),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn new(path: &Path) -> Self {
        let metadata = path.metadata().unwrap();
        let (width, height) = get_image_size(path);
        Self {
            weight: metadata.len(),
            width,
            height,
            rotation: get_rotation_code(path),
            created: metadata.created().ok(),
            modified: metadata.modified().ok(),
            accessed: metadata.accessed().ok(),
        }
    }
}

fn get_image_size(path: &Path) -> (usize, usize){
    match imagesize::size(path) {
        Ok(size) => (size.width, size.height),
        Err(_) => (0,0)
    }
}
fn get_rotation_code(path: &Path) -> Option<u32> {
    let file = File::open(path).expect("Could not open file");
    let mut bufreader = BufReader::new(file);
    let exifreader = exif::Reader::new();
    if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
        return read_exif(exif);
    }
    None
}

fn read_exif(exif: Exif) -> Option<u32> {
    let orientation = exif.get_field(Tag::Orientation, In::PRIMARY);
    match orientation {
        Some(orientation) => {
            let orientation = orientation.value.get_uint(0).unwrap();
            Some(orientation)
        }
        None => None,
    }
}
