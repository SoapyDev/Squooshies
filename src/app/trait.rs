use dioxus::prelude::{Element};

pub trait ToHtml{
    fn to_html(&self) -> Element;
}

