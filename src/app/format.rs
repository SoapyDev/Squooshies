use std::path::PathBuf;
use image::{DynamicImage, ImageFormat};
use crate::components::ToHtml;
use dioxus::prelude::*;
use ravif::{Encoder, Img};
use rgb::{FromSlice, RGBA8};
use crate::error::TransformationError;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Format {
    pub image : Option<ImageFormat>,
    pub quality : Quality,
    pub speed : Speed
}

const OPTIONS: [(&str, &str); 6] = [("none", "No reformating"),("png", "PNG"), ("jpg", "JPG"), ("webp", "WEBP"), ("avif", "AVIF"), ("tiff", "TIFF")];
impl ToHtml for ImageFormat{
    fn to_html(&self) -> Element {
        rsx!{
            for (value, label) in OPTIONS {
                option{
                    value: value,
                    {label}
                }
            }
        }
    }
}

impl Format{
    pub fn get_default_image_format() -> ImageFormat{
        ImageFormat::Png
    }
    pub fn set_format(&mut self, value: String){
        self.image = match value.as_str(){
            "png" => Some(ImageFormat::Png),
            "jpg" => Some(ImageFormat::Jpeg),
            "webp" => Some(ImageFormat::WebP),
            "avif" => Some(ImageFormat::Avif),
            "tiff" => Some(ImageFormat::Tiff),
            _ => None
        }
    }
    
    pub fn apply(&self, image: &mut DynamicImage, path: PathBuf) -> Result<(), TransformationError>{
        match self.image {
            Some(ImageFormat::Png) | Some(ImageFormat::Jpeg) | Some(ImageFormat::Tiff) => {
                let format = self.image.unwrap();
                save_image_with_format(image, path, format)
            }
            Some(ImageFormat::WebP) => save_image_as_webp(image, path, &self.quality),
            Some(ImageFormat::Avif) => save_image_as_avif(image, path, &self.quality, &self.speed),
            _ => {Ok(())},
        }
    }
}

fn save_image_with_format(image: &mut DynamicImage, path: PathBuf, format: ImageFormat) -> Result<(), TransformationError>{
    let extension = format.extensions_str().first().unwrap_or(&"png");
    let path = path.with_extension(extension);
    image
        .save_with_format(path, format)
        .map_err(|err| TransformationError::Format(err.to_string())
        )
}


fn save_image_as_webp(image: &mut DynamicImage, path: PathBuf, quality: &Quality) -> Result<(), TransformationError>{
    let path = path.with_extension("webp");
    let webp = webp::Encoder::from_image(&image)
        .map_err(|err| TransformationError::Format(err.to_string()))?
        .encode(quality.value as f32);

    std::fs::write(path, &*webp)
        .map_err(|err| TransformationError::Format(err.to_string()))
}

fn save_image_as_avif(image: &mut DynamicImage, path: PathBuf, quality: &Quality, speed: &Speed) -> Result<(), TransformationError>{
    let path = path.with_extension("avif");
    let raw = image.to_rgba8().into_raw();
    let rgb: &[RGBA8] = raw.as_rgba();
    let avif = Encoder::new()
        .with_speed(speed.value)
        .with_quality(quality.value as f32)
        .encode_rgba(Img::new(
            rgb,
            image.width().try_into().unwrap(),
            image.height().try_into().unwrap(),
        ))
        .map_err(|err| TransformationError::Format(err.to_string()))?;

    std::fs::write(
        path,
        avif.avif_file
    ).map_err(|err| TransformationError::Format(err.to_string()))
}

#[derive(Clone, PartialEq, Debug)]
pub struct Quality{
    pub value: u8
}

impl Quality {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Quality{
    fn default() -> Self {
        Self { value: 75 }
    }
}

impl From<String> for Quality{
    fn from(value: String) -> Self {
        let quality = value.parse::<u8>().unwrap_or(75);
        Self { value: quality }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Speed {
    pub value: u8
}

impl Speed {
    pub fn new() -> Self{
        Self::default()
    }
}

impl Default for Speed {
    fn default() -> Self {
        Self { value: 7 }
    }
}

impl From<String> for Speed {
    fn from(value: String) -> Self {
        let effort = value.parse::<u8>().unwrap_or(7);
        Self { value: effort }
    }
}
