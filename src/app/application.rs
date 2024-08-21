use crate::app::format::Format;
use crate::app::paths::Paths;
use crate::app::picture::Picture;
use crate::app::resize::Resize;
use crate::app::rotate::Rotate;
use crate::app::sort::Sort;
use crate::error::TransformationError;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelBridge, ParallelIterator};
use std::path::{Path, PathBuf};

#[derive(Clone, PartialEq, Debug)]
pub struct Application {
    pub paths: Paths,
    pub resize: Resize,
    pub format: Format,
    pub rotate: Rotate,
    pub sort: Sort,
    pub pictures: Vec<Picture>,
    pub errors: Vec<TransformationError>,
    pub is_in_process: bool,
    pub is_processed: bool,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            paths: Paths::default(),
            resize: Resize::default(),
            format: Format::default(),
            rotate: Rotate::default(),
            sort: Sort::default(),
            pictures: Vec::new(),
            errors: Vec::new(),
            is_in_process: false,
            is_processed: false,
        }
    }
}

impl Application {
    pub fn sort_pictures(&mut self) {
        self.sort.apply(&mut self.pictures);
    }
    pub fn get_pictures(&'static self) -> &'static Vec<Picture> {
        &self.pictures
    }
    pub fn set_source_path(&mut self, path: Option<PathBuf>) {
        if let Some(path) = path {
            self.paths.source = path;
            let _ = self.fetch_pictures();
        }
    }

    pub fn set_destination_path(&mut self, path: Option<PathBuf>) {
        if let Some(path) = path {
            self.paths.destination = path;
        }
    }

    pub async fn transform(&mut self) -> Result<(), std::io::Error> {
        let resize = &self.resize;
        let rotate = &self.rotate;
        let format = &self.format;
        let destination = self.paths.destination.to_owned();

        self.pictures.par_iter_mut().for_each(|picture| {
            if !picture.is_selected {
                return;
            }

            picture.is_in_process = true;

            if let Ok(mut image) = picture.load() {
                resize.apply(&mut image);

                rotate.apply(&mut image, picture.metadata.rotation);

                let path = picture.name.build(&destination);
                if let Err(e) = format.apply(&mut image, path) {
                    println!("Could not format : {}", picture.get_name());
                    println!("{}", e);
                }
            } else {
                println!("Could not get image : {}", picture.get_name());
            }

            picture.is_in_process = false;
            picture.is_processed = true;
        });

        Ok(())
    }


    fn fetch_pictures(&mut self) -> Result<(), std::io::Error> {
        let path = &self.paths.source;

        let pictures = path
            .read_dir()?
            .par_bridge()
            .filter_map(|result| if let Ok(val) = result {
                let path = val.path();
                if Self::is_image(&path) {
                    Some(Picture::new(path))
                } else { None }
            } else { None })
            .collect();

        self.pictures = pictures;
        self.sort_pictures();

        Ok(())
    }
    fn is_image(path: &Path) -> bool {
        match path.extension() {
            Some(ext) => {
                let ext = ext.to_str().unwrap_or_default().to_lowercase();
                ext == "jpg" || ext == "png" || ext == "jpeg" || ext == "webp" || ext == "avif" || ext == "tiff"
            }
            None => false
        }
    }

    pub fn select_all(&mut self) {
        self.pictures.iter_mut().for_each(|p| p.is_selected = true);
    }
    pub fn unselect_all(&mut self) {
        self.pictures.iter_mut().for_each(|p| p.is_selected = false);
    }

    pub fn is_all_selected(&self) -> bool {
        self.pictures.iter().all(|p| p.is_selected)
    }
}