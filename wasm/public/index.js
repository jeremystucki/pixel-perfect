const js = import("./node_modules/pixel-perfect/pixel_perfect_wasm.js");

js.then(js => {
    document.getElementById("button").onclick = function() {
        let reader = new FileReader();
        reader.onload = function() {
            let pixelSize = document.getElementById("pixel-size").valueAsNumber;

            let uint8Array = new Uint8Array(this.result);
            let output = pixelSize.isNaN ? js.export_normally(uint8Array) : js.force_export(uint8Array, pixelSize);

            let a = window.document.createElement("a");
            a.href = window.URL.createObjectURL(new Blob([output], { type: "image/png" }));
            a.download = "image.png";

            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
        };

        let file = document.getElementById("file").files[0];
        reader.readAsArrayBuffer(file);
    };
});
