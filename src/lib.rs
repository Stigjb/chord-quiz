#![recursion_limit = "512"]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_lossless, clippy::non_ascii_literal)]

mod app;
mod chord;
mod score;
mod settings;
mod tpc_octave;

use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Entry point for the web app
#[wasm_bindgen]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}
