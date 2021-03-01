//#![feature(custom_inner_attributes)]
//#![no_std]

use wasm_bindgen::prelude::*;
use web_sys::console;
//use ux_charts::WebCanvas;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is the entry point for the web app
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  wasm_logger::init(wasm_logger::Config::default());

  console::log_1(&"Hello ux-charts".into());

  Ok(())
}
