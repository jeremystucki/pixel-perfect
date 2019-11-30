const js = import('./node_modules/pixel-perfect/pixel_perfect_wasm.js');

js.then(js => {
    document.getElementById('button').addEventListener('click', () => {
        let pixelSize = document.getElementById('pixel-size').valueAsNumber;
        document.getElementById('file').files[0].arrayBuffer().then((arrayBuffer) => {
            let uint8Array = new Uint8Array(arrayBuffer);
            let output = isNaN(pixelSize) ? js.export_normally(uint8Array) : js.force_export(uint8Array, pixelSize);

            let a = document.createElement('a');
            a.href = URL.createObjectURL(new Blob([output], { type: 'image/png' }));
            a.download = 'image.png';

            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
        });
    });
});
