import init, { load_openscad, get_camera_info } from '../pkg/rust_raytracer_wasm.js';

const BLOCK_SIZE = 50;
const worker = new Worker('worker.js', { type: 'module' });

const input = `
// camera
camera(
    // aspect_ratio = 1.0,
    image_width = 400,
    image_height = 400,
    samples_per_pixel = 10,
    max_depth = 10,
    vertical_fov = 90.0,
    look_from = [50.0, -50.0, 70.0],
    look_at = [0.0, 0.0, 0.0],
    up = [0.0, 0.0, 1.0],
    defocus_angle = 0.0,
    focus_distance = 10.0,
    background = [0.7, 0.8, 1.0]
);

color([0,125,255]/255)
    scale([1.2,1,1])
    cube([60,20,10],center=true);
`;

worker.onmessage = (ev) => {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    const { xmin, xmax, ymin, ymax, results } = ev.data;
    let i = 0;
    for (let y = ymin; y < ymax; y++) {
        for (let x = xmin; x < xmax; x++) {
            const color = results[i++];
            const { r, g, b } = color;
            ctx.fillStyle = `rgb(${r},${g},${b})`;
            ctx.fillRect(x, y, 1, 1);
        }
    }
};

worker.onerror = (err) => {
    console.error(`worker error`, err);
};

async function render() {
    await init();
    load_openscad(input);

    const cameraInfo = get_camera_info();

    const canvas = document.getElementById('canvas');
    canvas.width = cameraInfo.width;
    canvas.height = cameraInfo.height;
    console.log(`render ${canvas.width}x${canvas.height}`);

    for (let y = 0; y < canvas.height; y += BLOCK_SIZE) {
        for (let x = 0; x < canvas.width; x += BLOCK_SIZE) {
            worker.postMessage({
                input,
                xmin: x,
                xmax: Math.min(canvas.width, x + BLOCK_SIZE),
                ymin: y,
                ymax: Math.min(canvas.height, y + BLOCK_SIZE),
            });
        }
    }
}

render();
