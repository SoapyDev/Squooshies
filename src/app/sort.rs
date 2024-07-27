use crate::app::picture::Picture;
use crate::components::ToHtml;
use dioxus::prelude::*;
#[derive(Clone, PartialEq, Debug)]
pub struct Sort{
    pub field: SortType,
    pub order: SortOrder
}


impl Default for Sort {
    fn default() -> Self {
        Self { field: SortType::Name, order: SortOrder::Asc }
    }
}

impl Sort {
    pub fn apply(&self, pictures: &mut Vec<Picture>) {
        match (&self.field, &self.order) {
            (SortType::Name, SortOrder::Asc) => pictures.sort_by(|a, b| a.get_name().cmp(&b.get_name())),
            (SortType::Name, SortOrder::Desc) => pictures.sort_by(|a, b| b.get_name().cmp(&a.get_name())),
            (SortType::Weight, SortOrder::Asc) => pictures.sort_by(|a, b| a.metadata.weight.cmp(&b.metadata.weight)),
            (SortType::Weight, SortOrder::Desc) => pictures.sort_by(|a, b| b.metadata.weight.cmp(&a.metadata.weight)),
            (SortType::Created, SortOrder::Asc) => pictures.sort_by(|a,b | a.metadata.created.cmp(&b.metadata.created)),
            (SortType::Created, SortOrder::Desc) => pictures.sort_by(|a,b | b.metadata.created.cmp(&a.metadata.created)),
            (SortType::Modified, SortOrder::Asc) => pictures.sort_by(|a, b| a.metadata.modified.cmp(&b.metadata.modified)),
            (SortType::Modified, SortOrder::Desc) => pictures.sort_by(|a, b| b.metadata.modified.cmp(&a.metadata.modified)),
            (SortType::Accessed, SortOrder::Asc) => pictures.sort_by(|a, b| a.metadata.accessed.cmp(&b.metadata.accessed)),
            (SortType::Accessed, SortOrder::Desc) => pictures.sort_by(|a, b| b.metadata.accessed.cmp(&a.metadata.accessed)),
        }
    }
    
    pub fn set_field(&mut self, value: String) {
        self.field = SortType::from(value);
    }
    
    pub fn set_order(&mut self) {
        self.order = if self.order == SortOrder::Asc {
            SortOrder::Desc
        }else{
            SortOrder::Asc
        };
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum SortType{
    #[default]
    Name,
    Weight,
    Created,
    Modified,
    Accessed
}

const SORT_TYPE : [(&str, &str); 5] = [("Name", "name"), ("Weight", "weight"), ("Created", "created"), ("Modified", "modified"), ("Accessed", "accessed")]; 

impl ToHtml for SortType {
    fn to_html(&self) -> Element {
        rsx!{
            for (label , value) in SORT_TYPE.iter() {
                option { value: *value, {label} }
            }
        }
    }
}

impl From<String> for SortType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "name" => Self::Name,
            "weight" => Self::Weight,
            "created" => Self::Created,
            "modified" => Self::Modified,
            "accessed" => Self::Accessed,
            _ => Self::Name,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum SortOrder{
    #[default]
    Asc,
    Desc
}



