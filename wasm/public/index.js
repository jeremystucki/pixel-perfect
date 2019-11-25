const js = import("./node_modules/pixel-perfect/pixel_perfect_wasm.js");

js.then(js => {
    document.getElementById("button").onclick = function() {
        let reader = new FileReader();
        reader.onload = function() {
            let output = js.handle_input(new Uint8Array(this.result));
            let a = window.document.createElement("a");
            a.href = window.URL.createObjectURL(new Blob([output], { type: 'image/png' }));
            a.download = "image.png";

            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
        };

        let file = document.getElementById("file").files[0];
        reader.readAsArrayBuffer(file);
    };
});
