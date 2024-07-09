#![feature(iter_collect_into)]

use dioxus::prelude::*;
use picturust_dx::App;

mod error;
mod components;
mod app;

fn main() {
    launch(App);
}