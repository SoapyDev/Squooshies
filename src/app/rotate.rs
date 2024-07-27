use dioxus::prelude::*;
use image::DynamicImage;
use crate::components::ToHtml;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Rotate{
    pub angle: Angle,
}

impl ToHtml for Rotate{
    fn to_html(&self) -> Element {
        rsx!{
            {self.angle.to_html()}
        }
    }
}

impl Rotate{
    pub fn set_angle(&mut self, value: String){
        self.angle = match value.as_str(){
            "none" => Angle::None,
            "90" => Angle::Quarter,
            "180" => Angle::Half,
            "270" => Angle::ThreeQuarters,
            _ => Angle::None
        }
    }

    pub fn apply(&self, image: &mut DynamicImage){
        match self.angle{
            Angle::None => {},
            Angle::Quarter => {*image =  image.rotate90();},
            Angle::Half => {*image =  image.rotate180();},
            Angle::ThreeQuarters => {*image =  image.rotate270();}
        }
    }
}

const OPTIONS: [(&str, &str); 4] = [("none", "0 deg"), ("90", "90 deg"), ("180", "180 deg"), ("270", "270 deg")];
#[derive(Clone, PartialEq, Debug, Default)]
pub enum Angle{
    #[default]
    None,
    Quarter,
    Half,
    ThreeQuarters
}

impl ToHtml for Angle{
    fn to_html(&self) -> Element {
        rsx!{
            for (value , label) in OPTIONS {
                option { value, {label} }
            }
        }
    }
}
