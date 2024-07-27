use dioxus::prelude::*;

#[component]
pub fn Checkbox(is_checked: bool, on_click: EventHandler<bool>,) -> Element {
    rsx!{
        div { class: "inline-flex items-center absolute top-2 right-2",

            label { class: "relative flex items-center p-3 rounded-full cursor-pointer",
                input {
                    class: "before:content[''] peer relative h-8 w-8 cursor-pointer appearance-none rounded-full border border-green-500 bg-transparent transition-all before:absolute before:top-2/4 before:left-2/4 before:block before:h-12 before:w-12 before:-translate-y-2/4 before:-translate-x-2/4 before:rounded-full before:bg-blue-gray-500 before:opacity-0 before:transition-opacity checked:border-green-600 checked:bg-green-600 checked:before:bg-green-600 hover:scale-105 hover:before:opacity-0",
                    r#type: "checkbox",
                    checked: is_checked,
                    onchange: move |evt| {
                        let checked = evt.checked();
                        on_click(checked)
                    }
                }
                span { class: "absolute text-white transition-opacity opacity-0 pointer-events-none top-2/4 left-2/4 -translate-y-2/4 -translate-x-2/4 peer-checked:opacity-100",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-4 w-4",
                        view_box: "0 0 20 20",
                        fill: "",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path {
                            fill_rule: "evenodd",
                            d: "M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z",
                            clip_rule: "evenodd"
                        }
                    }
                }
            }
        }
    }
}