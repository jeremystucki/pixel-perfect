const js = import("./node_modules/pixel-perfect/pixel_perfect_wasm.js");
js.then(js => {
    js.greet("WebAssembly");
});
