const js = import("./node_modules/pixel-perfect/pixel_perfect_wasm.js");

js.then(js => {
    document.getElementById("button").onclick = function() {
        let reader = new FileReader();
        reader.onload = function() {
            js.handle_input(new Uint8Array(this.result));
        };

        let file = document.getElementById("file").files[0];
        reader.readAsArrayBuffer(file);
    };
});
