use dioxus::prelude::*;
use image::DynamicImage;
use image::imageops::FilterType;
use crate::components::ToHtml;

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Resize {
    pub resize_type: Option<ResizeType>,
    pub method: ResizeMethod,
    pub width: u32,
    pub height: u32
}

impl Resize{
    pub fn set_resize_type(&mut self, value: String){
        self.resize_type = match value.as_str(){
            "exact" => Some(ResizeType::Exact),
            "fill" => Some(ResizeType::Fill),
            "thumbnail" => Some(ResizeType::Thumbnail),
            _ => None
        }
    }
    pub fn set_method(&mut self, value: String){
        self.method = match value.as_str(){
            "lanczos3" => ResizeMethod::Lanczos3,
            "nearest" => ResizeMethod::Nearest,
            "catmullrom" => ResizeMethod::CatmullRom,
            "triangle" => ResizeMethod::Triangle,
            "gaussian" => ResizeMethod::Gaussian,
            _ => ResizeMethod::Lanczos3
        }
    }
    
    pub fn apply(&self, image: &mut DynamicImage){
        if let Some(resize_type) = &self.resize_type{
            match resize_type{
                ResizeType::Fill => {*image = image.resize_to_fill(self.width, self.height, self.method.to_image_filter());}
                ResizeType::Exact => {*image = image.resize_exact(self.height, self.height, self.method.to_image_filter());}
                ResizeType::Thumbnail => {*image = image.thumbnail(self.width, self.height);}
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ResizeType{
    #[default]
    Exact,
    Fill,
    Thumbnail
}

const RESIZE_TYPES: [(&str, &str); 3] = [("exact", "Exact"), ("fill", "Fill"), ("thumbnail", "Thumbnail")];

impl ToHtml for ResizeType{
    fn to_html(&self) -> Element {
        rsx!{
            for (value, label) in RESIZE_TYPES.iter() {
                option{
                    value: "{value}",
                    {label}
                }
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Default)]
pub enum ResizeMethod{
    #[default]
    Lanczos3,
    Nearest,
    CatmullRom,
    Triangle,
    Gaussian
}

impl ResizeMethod{
    pub fn to_image_filter(&self) -> FilterType{
        match self{
            ResizeMethod::Lanczos3 => FilterType::Lanczos3,
            ResizeMethod::Nearest => FilterType::Nearest,
            ResizeMethod::CatmullRom => FilterType::CatmullRom,
            ResizeMethod::Triangle => FilterType::Triangle,
            ResizeMethod::Gaussian => FilterType::Gaussian
        }
    }
}

const RESIZE_METHODS: [(&str, &str); 5] = [("lanczos3", "Lanczos3"), ("nearest", "Nearest"), ("catmullrom", "CatmullRom"), ("triangle", "Triangle"), ("gaussian", "Gaussian")];

impl ToHtml for ResizeMethod{
    fn to_html(&self) -> Element {
        rsx!{
            for (value, label) in RESIZE_METHODS.iter() {
                option{
                    value: "{value}",
                    {label}
                }
            }
        }
    }
}