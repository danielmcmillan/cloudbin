import("./pkg")
  .catch((e) => console.error("Failed to load the wasm module", e))
  .then((wasm) => {
    wasm.run_app();
  });
