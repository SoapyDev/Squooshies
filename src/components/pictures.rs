use dioxus::prelude::*;
use crate::app::{Application, Picture};
use crate::components::Checkbox;

#[component]
pub fn Pictures(app: Signal<Application>) -> Element {
    
    rsx!{
        for (index , picture) in app().pictures.into_iter().enumerate() {
            Picture { picture, index, app }
        }
    }
}


#[component]
fn Picture(picture: Picture, index: usize, app: Signal<Application>) -> Element {
    
    rsx! {
        figure { key: "{picture.path.to_str().unwrap_or_default()}", class: "w-80 h-full flex flex-col gap-8",
            div {
                class: "w-full h-full shadow-2xl drop-shadow-2xl shadow-slate-600 relative",
                onclick: move |_| {
                    let state = !app.with(|a| a.pictures[index].is_selected);
                    app.with_mut(|a| a.pictures[index].is_selected = state);
                },
                img {
                    src: picture.get_path(),
                    loading: "lazy",
                    width: "320px",
                    height: "320px",
                    class: "w-80 h-80 object-cover object-center rounded-lg"
                }
                Checkbox {
                    is_checked: picture.is_selected,
                    on_click: move |evt| {
                        if evt {
                            app.with_mut(|a| a.pictures[index].is_selected = true);
                        } else {
                            app.with_mut(|a| a.pictures[index].is_selected = false);
                        }
                    }
                }
            }
            figcaption { class: "w-full p-4 text-center flex flex-col gap-2",
                div { class: "w-full text-slate-400", {picture.get_name()} }
                div { class: "w-full text-slate-500", {picture.get_weight()} }
                div { class: "w-full text-slate-500", {picture.get_size()} }
            }
        }
    }
}
