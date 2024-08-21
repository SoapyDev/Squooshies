use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaFolderPlus;
use dioxus_free_icons::Icon;
use std::path::PathBuf;

#[component]
pub fn FileSelector(label: &'static str, value: PathBuf, on_change: EventHandler<String>, on_click: EventHandler<()>) -> Element {
    rsx! {
        div { class: "w-full my-8 flex flex-row flex-nowrap border border-slate-700 rounded-lg hover:border-blue-500 has-[:focus]:border-blue-500 transition-colors duration-250 ease-in-out",
            button {
                class: "text-slate-200 p-4 bg-gray-900 rounded-l-lg hover:text-blue-500 focus:outline-none focus:ring-0 transition-all duration-250 ease-in-out",
                onclick: move |_| on_click(()),
                Icon { icon: FaFolderPlus, width: 30, height: 30 }
            }
            input {
                r#type: "text",
                class: "w-full rounded-r-lg text-slate-200 bg-gray-900 p-4 focus:outline-none focus:ring-0",
                value: value.to_string_lossy().to_string(),
                oninput: move |evt| on_change.call(evt.value()),
                placeholder: "{label}"
            }
        }
    }
}