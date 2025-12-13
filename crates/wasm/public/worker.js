import init, { load_openscad, render } from '../pkg/rust_raytracer_wasm.js';

let wasmReady = init();

onmessage = async (ev) => {
    try {
        const { input, xmin, xmax, ymin, ymax } = ev.data;
        await wasmReady;
        load_openscad(input);

        const results = render(xmin, xmax, ymin, ymax);

        postMessage({ xmin, xmax, ymin, ymax, results: results.map(c => ({ r: c.r, g: c.g, b: c.b })) });
    } catch (err) {
        postMessage({ error: `${err}` });
    }
};
