
import("../../pkg").then(module => {
  // we don't need to explicitly tun app,
  // because in module used #[wasm_bindgen(start)]
  //module.run_app();
});
