use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use picturust_dx::App;

mod error;
mod components;
mod app;

fn main() {

    LaunchBuilder::new()
        .with_cfg(
            Config::new()
                .with_background_color((2, 6, 23,100))
                .with_window(
                    WindowBuilder::new()
                        .with_always_on_top(false)
                        .with_maximized(true)
                )
                .with_disable_context_menu(true)
        )
        .launch(App);
}
