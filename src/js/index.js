import "../scss/styles.scss";
import("../../pkg")
  .then(module => {})
  .catch(reason => {
    console.error('Wasm module not loaded', reason);
  });
