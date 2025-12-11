import init, { render_openscad } from '../pkg/rust_raytracer_wasm.js';

let wasmReady = init();

onmessage = async (ev) => {
    try {
        const { input, xmin, xmax, ymin, ymax } = ev.data;
        await wasmReady;

        const results = render_openscad(input, xmin, xmax, ymin, ymax);

        postMessage({ xmin, xmax, ymin, ymax, results });
    } catch (err) {
        postMessage({ error: `${err}` });
    }
};
