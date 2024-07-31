use std::path::PathBuf;
use dioxus::prelude::*;
use image::ImageFormat;
use crate::components::{Checkbox, FileSelector, Numbers, OrderByButton, Pictures, TransformButton};
use crate::components::{Selectable, SelectableSetting};
use crate::app::{Application, SortType, SortOrder, ResizeType, ResizeMethod, Format, Quality, Speed, Rotate};

mod app;
mod error;
mod components;

#[component]
pub fn App() -> Element {

    let mut app = use_signal(Application::default);
    let mut transform = use_future(move || async move {
        if !app().paths.is_valid() {
            return;
        }
        app().transform().await.expect("Could not transform pictures");
    });
    
    rsx! {
        style{{include_str!("../public/output.css")}}
            body { class: "dark h-screen w-screen overflow-hidden p-0 m-0 flex bg-slate-950",
            section { class: "w-full min-w-96 overflow-y-auto overflow-performance relative",
                header { class: "w-full py-4 px-8 flex justify-end align-center gap-8 sticky top-0 left-0 bg-slate-950 z-20",
                    Checkbox {
                        is_checked: app.with(|a| a.is_all_selected()),
                        on_click: move |evt| {
                            if evt {
                                app.with_mut(|a| a.select_all())
                            } else {
                                app.with_mut(|a| a.unselect_all())
                            }
                        }
                    }
                    Selectable {
                        options: SortType::default(),
                        label: "Sort by : ",
                        on_change: move |evt| {
                            app.with_mut(|a| a.sort.set_field(evt));
                            app.with_mut(|a| a.sort_pictures());
                        }
                    }
                    OrderByButton {
                        is_asc: app.with(|a| a.sort.order == SortOrder::Asc),
                        on_click: move |_| {
                            app.with_mut(|a| a.sort.set_order());
                            app.with_mut(|a| a.sort_pictures());
                        }
                    }
                }
                div { class: "w-full h-fit p-8 m-0 flex flex-row flex-wrap gap-16",
                    Pictures { app }
                }
            }
            section { class: "w-1/4 min-w-96 h-screen p-8 m-0 sticky top-0 bg-gray-900 overflow-y-auto",
                FileSelector {
                    value: app.with(|a| a.paths.source.clone()),
                    label: "Source path",
                    on_click: move |_| {
                        let files = rfd::FileDialog::new()
                            .set_title("Select a source directory")
                            .set_directory(".")
                            .pick_folder();
                        app.with_mut(|a| a.set_source_path(files));
                    },
                    on_change: move |evt| {
                        let path = PathBuf::from(evt);
                        if path.is_dir() {
                            app.with_mut(|a| a.set_source_path(Some(path)));
                        }
                    }
                }
                FileSelector {
                    value: app.with(|a| a.paths.destination.clone()),
                    label: "Output path",
                    on_click: move |_| {
                        let files = rfd::FileDialog::new()
                            .set_title("Select an output directory")
                            .set_directory(".")
                            .pick_folder();
                        app.with_mut(|a| a.set_destination_path(files));
                    },
                    on_change: move |evt| {
                        let path = PathBuf::from(evt);
                        if path.is_dir() {
                            app.with_mut(|a| a.set_destination_path(Some(path)));
                        }
                    }
                }

                SelectableSetting {
                    options: ResizeType::default(),
                    label: "Resize : ",
                    on_change: move |evt| {
                        app.with_mut(|a| a.resize.set_resize_type(evt));
                    }
                }
                if app.with(|a| a.resize.resize_type.is_some()) {
                    SelectableSetting {
                        options: ResizeMethod::default(),
                        label: "Method : ",
                        on_change: move |evt| {
                            app.with_mut(|a| a.resize.set_method(evt));
                        }
                    }
                    Numbers {
                        value: app.with(|a| a.resize.width),
                        min: 0,
                        max: 8192,
                        step: 1,
                        label: "Width : ",
                        on_change: move |evt: String| {
                            let value = evt.parse::<u32>().unwrap_or(0);
                            app.with_mut(|a| a.resize.width = value);
                        }
                    }
                    Numbers {
                        value: app.with(|a| a.resize.height),
                        min: 0,
                        max: 8192,
                        step: 1,
                        label: "Height : ",
                        on_change: move |evt: String| {
                            let value = evt.parse::<u32>().unwrap_or(0);
                            app.with_mut(|a| a.resize.height = value);
                        }
                    }
                }
                SelectableSetting {
                    options: Format::get_default_image_format(),
                    label: "Format : ",
                    on_change: move |evt| {
                        app.with_mut(|a| a.format.set_format(evt));
                    }
                }

                if app.with(|a| {
                    a.format.image == Some(ImageFormat::Avif)
                        || a.format.image == Some(ImageFormat::WebP)
                })
                {
                    div {
                        label { class: "w-full p-4 text-slate-200 my-4",
                            {format!("Quality : {}", app.with(|a| a.format.quality.value))}
                        }
                        input {
                            r#type: "range",
                            class: "w-full p-4",
                            min: 0,
                            max: 100,
                            value: app.with(|a| a.format.quality.value).to_string(),
                            onchange: move |evt| {
                                let value = evt.value();
                                app.with_mut(|a| a.format.quality = Quality::from(value));
                            }
                        }
                    }

                    div {
                        label { class: "w-full p-4 text-slate-200 my-4",
                            {format!("Speed : {}", app.with(|a| a.format.speed.value))}
                        }
                        input {
                            r#type: "range",
                            class: "w-full p-4",
                            min: 1,
                            max: 10,
                            value: app.with(|a| a.format.speed.value).to_string(),
                            onchange: move |evt| {
                                let value = evt.value();
                                app.with_mut(|a| a.format.speed = Speed::from(value));
                            }
                        }
                    }
                }

                SelectableSetting {
                    options: Rotate::default(),
                    label: "Rotate : ",
                    on_change: move |evt| {
                        app.with_mut(|a| a.rotate.set_angle(evt));
                    }
                }
                TransformButton {
                    is_disabled: app.with(|a| !a.paths.is_valid()),
                    on_click: move |_| {
                        transform.restart();
                    }
                }
            }
        }
    }
}
