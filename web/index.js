import "./styles.scss";
import("../pkg").catch(reason => {
  console.error('Wasm module not loaded', reason);
});
