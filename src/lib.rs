#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Document, HtmlCanvasElement, CanvasRenderingContext2d};
use ux_charts::WebCanvas;
use ux_primitives::canvas::CanvasContext;

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

  let (_, context_web) = get_web_canvas(&document, "web-canvas");

  context_web.begin_path();

  context_web.arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
  context_web.move_to(110.0, 75.0);

  context_web.arc(75.0, 75.0, 35.0, 0.0, std::f64::consts::PI).unwrap(); // Mouth (clockwise)
  context_web.move_to(65.0, 65.0);

  context_web.arc(60.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0).unwrap(); // Left eye
  context_web.move_to(95.0, 65.0);

  context_web.arc(90.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0).unwrap(); // Right eye
  context_web.stroke();

  let (_, context_ux) = get_web_canvas(&document, "ux-canvas");
  let ux_canvas = WebCanvas::new(context_ux);

  ux_canvas.begin_path();

  ux_canvas.arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0, true);
  ux_canvas.move_to(110.0, 75.0);

  ux_canvas.arc(75.0, 75.0, 35.0, 0.0, std::f64::consts::PI, false); // Mouth (clockwise)
  ux_canvas.move_to(65.0, 65.0);

  ux_canvas.arc(60.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0, true); // Left eye
  ux_canvas.move_to(95.0, 65.0);

  ux_canvas.arc(90.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0, true); // Right eye
  // ux_canvas.stroke();

  Ok(())
}

fn get_web_canvas(document: &Document, id: &str) -> (HtmlCanvasElement, CanvasRenderingContext2d) {
  let canvas = document.get_element_by_id(id)
    .expect("#app-canvas not found")
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .expect("no canvas element");

  let context = canvas.get_context("2d").unwrap().unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

  (canvas, context)
}
