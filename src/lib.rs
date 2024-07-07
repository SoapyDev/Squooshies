use std::path::PathBuf;
use dioxus::prelude::*;
use image::ImageFormat;
use manganis::mg;
use crate::components::{Checkbox, FileSelector, Numbers, OrderByButton, TransformButton};
use crate::components::{Selectable, SelectableSetting};
use crate::app::{Application, SortType, SortOrder, ResizeType, ResizeMethod, Format, Quality, Speed, Rotate};

mod app;
mod error;
mod components;

const _TAILWIND_URL: &str = mg!(file("./src/output.css"));

#[component]
pub fn App() -> Element {

    let mut app = use_signal(Application::default);
    rsx! {
        style{ "_TAILWIND_URL" }
        body{
            class: "h-screen w-screen overflow-auto  p-0 m-0 flex bg-slate-950",
            section{
                class: "w-full min-w-96",
                header{
                    class: "w-full h-16 py-4 px-8 flex justify-end align-center gap-8 sticky top-0 bg-slate-950",
                    Checkbox{
                        is_checked: app.with(|a| a.is_all_selected()),
                        on_click: move |evt| {
                            if evt{
                                app.with_mut(|a| a.select_all())
                            }else{
                                app.with_mut(|a| a.unselect_all())
                            }
                        }
                    },
                    Selectable{
                        options: SortType::default(),
                        label: "Sort by : ",
                        on_change: move |evt| {
                            app.with_mut(|a| a.sort.set_field(evt));
                            app.with_mut(|a| a.sort_pictures());
                        }
                    },
                    OrderByButton{
                        is_asc : app.with(|p| p.sort.order == SortOrder::Asc),
                        on_click: move |_| {
                            app.with_mut(|a| a.sort.set_order());
                            app.with_mut(|a| a.sort_pictures());
                        }
                    }
                },
                div{
                    class: "w-full h-fit p-8 m-0 flex flex-row flex-wrap gap-16",
                    for (index,  picture) in app().pictures.iter().enumerate(){
                        figure{
                            class: "w-80 h-full flex flex-col gap-8",
                            div{
                                class: "w-full h-full shadow-2xl drop-shadow-2xl shadow-slate-600 relative",
                                onclick: move |_| {
                                    let state = !app.with(|a| a.pictures[index].is_selected);
                                    app.with_mut(|a| a.pictures[index].is_selected = state);
                                },
                                img{
                                    src: picture.path.to_str().unwrap(),
                                    loading: "lazy",
                                    width: "320px",
                                    height: "320px",
                                    class: "w-80 h-80 object-cover object-center rounded-lg",
                                }
                                Checkbox{
                                    is_checked: picture.is_selected,
                                    on_click: move |evt| {
                                        if evt{
                                            app.with_mut(|a| a.pictures[index].is_selected = true);
                                        }else{
                                            app.with_mut(|a| a.pictures[index].is_selected = false);
                                        }
                                    }
                                }
                                div{
                                    class: "w-full h-full absolute top-0 left-0 bg-black/40",
                                    class: if picture.is_in_process {"z-10"} else {"hidden z-0"},
                                    svg{
                                        view_box: "0 0 800 800",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        circle{
                                            class: "animate-progress",
                                            cx: "400",
                                            cy: "400",
                                            r: "200",
                                            fill: "none",
                                            stroke: "white",
                                            stroke_dasharray: "250 1400",
                                            stroke_width: "25",
                                            stroke_linecap: "round"
                                        }
                                    }
                                }
                            }
                            figcaption{
                                class: "w-full p-4 text-center flex flex-col gap-2",
                                div{
                                    class: "w-full text-slate-400",
                                    {picture.get_name()}
                                }
                                div{
                                    class: "w-full text-slate-500",
                                    {picture.get_weight()}
                                }
                            }
                        }
                    }
                }
            },
            section{
                class: "w-1/4 h-screen p-8 m-0 sticky top-0 bg-gray-900",
                FileSelector{
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
                
                FileSelector{
                    value: app.with(|a| a.paths.destination.clone()),
                    label: "Output path",
                    on_click:move |_| {
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
                    },
                }

                SelectableSetting{
                    options: ResizeType::default(),
                    label: "Resize : ",
                    on_change: move |evt| {
                        app.with_mut(|a| a.resize.set_resize_type(evt));
                    },
                }
                if app.with(|a| a.resize.resize_type.is_some() ) {
                    SelectableSetting{
                        options: ResizeMethod::default(),
                        label: "Method : ",
                        on_change: move |evt| {
                            app.with_mut(|a| a.resize.set_method(evt));
                        },
                    }
                    Numbers{
                        value: app.with(|a| a.resize.width),
                        min: 0,
                        max: 8192,
                        step: 1,
                        label: "Width : ",
                        on_change: move |evt: String| {
                            let value = evt.parse::<u32>().unwrap_or(0);
                            app.with_mut(|a| a.resize.width = value);
                        },
                    }
                    Numbers{
                        value: app.with(|a| a.resize.height),
                        min: 0,
                        max: 8192,
                        step: 1,
                        label: "Height : ",
                        on_change: move |evt: String| {
                            let value = evt.parse::<u32>().unwrap_or(0);
                            app.with_mut(|a| a.resize.height = value);
                        },
                    }
                }
                SelectableSetting{
                    options: Format::get_default_image_format(),
                    label: "Format : ",
                    on_change: move |evt| {
                        app.with_mut(|a| a.format.set_format(evt));
                    },
                }

                if app.with(|a| a.format.image == Some(ImageFormat::Avif) || a.format.image == Some(ImageFormat::WebP)) {
                    div{
                        label{
                            class: "w-full p-4 text-slate-200 my-4",
                            {format!("Quality : {}", app.with(|a| a.format.quality.value))},
                        }
                        input{
                            r#type: "range",
                            class: "w-full p-4",
                            min: 0,
                            max: 100,
                            value: app.with(|a| a.format.quality.value).to_string(),
                            onchange: move |evt| {
                                let value = evt.value();
                                app.with_mut(|a| a.format.quality = Quality::from(value));
                            },
                        }
                    }

                    div{
                        label{
                            class: "w-full p-4 text-slate-200 my-4",
                            {format!("Speed : {}", app.with(|a| a.format.speed.value))},
                        }
                        input{
                            r#type: "range",
                            class: "w-full p-4",
                            min: 1,
                            max: 10,
                            value: app.with(|a| a.format.speed.value).to_string(),
                            onchange: move |evt| {
                                let value = evt.value();
                                app.with_mut(|a| a.format.speed = Speed::from(value));
                            },
                        }
                    }

                }

                SelectableSetting{
                    options: Rotate::default(),
                    label: "Rotate : ",
                    on_change: move |evt| {
                        app.with_mut(|a| a.rotate.set_angle(evt));
                    },
                }
                TransformButton{
                    is_disabled: app.with(|a| !a.paths.is_valid()),
                    on_click: move |_| {
                        let _ = app.with_mut(|a| a.transform());
                    }
                }
            }
        }
    }
}
