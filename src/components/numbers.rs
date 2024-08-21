use dioxus::prelude::*;

#[component]
pub fn Numbers<T: ToString + PartialEq + Clone + 'static>
(value: T, label: &'static str, min: T, max: T, step: T, on_change: EventHandler<String>) -> Element {
    rsx! {
        label { class: "w-full text-slate-200", {label} }
        div { class: "w-full text-slate-200 mb-8 mt-4 text-center bg-transparent border rounded-lg border-slate-700 has-[:focus]:border-blue-500 focus:border-blue-500 hover:border-blue-500",
            input {
                r#type: "number",
                class: "w-full p-4 bg-transparent appearance-none text-slate-200 focus:outline-none focus:ring-0 peer",
                value: "{value}",
                min: min.to_string(),
                max: max.to_string(),
                step: step.to_string(),
                onchange: move |evt| { on_change(evt.value()) }
            }
        }
    }
}