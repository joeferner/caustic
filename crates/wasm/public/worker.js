import init, { render } from '../pkg/rust_raytracer_wasm.js';

let wasmReady = init();

onmessage = async (ev) => {
    const { width, height, xmin, xmax, ymin, ymax } = ev.data;
    await wasmReady;
    const aspectRatio = width / height;
    const results = [];
    for (let y = ymin; y < ymax; y++) {
        for (let x = xmin; x < xmax; x++) {
            const color = render("threeBall", aspectRatio, width, x, y);
            results.push(color);
        }
    }
    postMessage({ xmin, xmax, ymin, ymax, results });
};
