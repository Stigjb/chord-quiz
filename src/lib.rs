#![recursion_limit = "512"]
#![warn(clippy::pedantic)]

mod app;
mod chord;
mod score;
mod tpc_octave;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Entry point for the web app
#[wasm_bindgen]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}
