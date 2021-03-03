#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
//use ux_charts::WebCanvas;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is the entry point for the web app
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  wasm_logger::init(wasm_logger::Config::default());

  console::log_1(&"Hello ux-charts".into());

  let window = web_sys::window().expect("no window");
  let document = window.document().expect("no document");
  //let body = document.body().expect("no body");

  let canvas = document.get_element_by_id("app-canvas")
    .expect("#app-canvas not found")
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .expect("no canvas element");

  let context = canvas.get_context("2d").unwrap().unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

  context.begin_path();

  context.arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
  context.move_to(110.0, 75.0);

  context.arc(75.0, 75.0, 35.0, 0.0, std::f64::consts::PI).unwrap(); // Mouth (clockwise)
  context.move_to(65.0, 65.0);

  context.arc(60.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0).unwrap(); // Left eye
  context.move_to(95.0, 65.0);

  context.arc(90.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0).unwrap(); // Right eye
  context.stroke();

  // let canvas: wss::HtmlCanvasElement = canvas::dyn_info::<wss::HtmlCanvasElement>
  Ok(())
}
