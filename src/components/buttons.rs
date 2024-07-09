use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaArrowDownWideShort, FaArrowUpShortWide};

#[component]
pub fn TransformButton(on_click: EventHandler<()>, is_disabled: bool) -> Element {
    rsx!{
        button{
            class: "w-full p-4 my-16 rounded-lg bg-slate-800 text-slate-200 hover:bg-slate-700 disabled:bg-slate-800 disabled:text-slate-400",
            onclick: move |_| on_click(()),
            disabled: is_disabled,
            "Apply changes",
        }
    }
}

#[component]
pub fn OrderByButton(is_asc: bool, on_click: EventHandler<()>) -> Element {
    rsx!{
        button{
            onclick: move |_| on_click(()),
            class: "p-4 h-fit mx-8 rounded-lg bg-transparent text-slate-400 hover:text-blue-500",
            if is_asc {
                Icon{
                   width: 25,
                   height: 24,
                   icon: FaArrowUpShortWide,
                }
            } else {
                Icon{
                   width: 25,
                   height: 25,
                   icon: FaArrowDownWideShort,
                }
            }
        }
    }
}