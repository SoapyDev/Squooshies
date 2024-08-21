use crate::components::traits::ToHtml;
use dioxus::prelude::*;

#[component]
pub fn Selectable<T: ToHtml + PartialEq + Clone + 'static>(options: T, label: &'static str, on_change: EventHandler<String>) -> Element {
    rsx! {
        div { class: "flex justify-center align-center gap-2 py-4",
            label { class: "h-fit text-sm font-medium dark:text-gray-400 text-center content-center w-full",
                {label}
            }
            select {
                onchange: move |evt| on_change.call(evt.value()),
                class: "appearance-none h-fit text-center text-sm text-gray-500 bg-transparent rounded-none dark:text-gray-400 focus:outline-none focus:ring-0 peer",
                {options.to_html()}
            }
        }
    }
}

#[component]
pub fn SelectableSetting<T: ToHtml + PartialEq + Clone + 'static>(options: T, label: &'static str, on_change: EventHandler<String>) -> Element {
    rsx! {
        label { class: "w-full text-slate-200", {label} }
        select {
            class: "w-full p-4 mb-8 mt-4 rounded-lg bg-transparent appearance-none text-gray-500 border border-slate-700 focus:outline-none focus:ring-0 peer hover:border-blue-500 focus:border-blue-500 text-center",
            onchange: move |evt| { on_change.call(evt.value()) },
            for option in options.to_html() {
                {option}
            }
        }
    }
}