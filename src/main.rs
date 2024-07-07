#![feature(iter_collect_into)]

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_logger::tracing::Level;
use picturust_dx::App;

mod error;
mod components;
mod app;

fn main() {
    // Get the system number of treads
    let  num_thread = num_cpus::get();
    rayon::ThreadPoolBuilder::new().num_threads(num_thread).build_global().unwrap();

    dioxus_logger::init(Level::INFO).expect("Failed to initialize logger");
    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(WindowBuilder::new().with_resizable(true)))
        .launch(App)
}