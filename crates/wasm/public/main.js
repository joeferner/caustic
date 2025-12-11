const BLOCK_SIZE = 10;
const worker = new Worker('worker.js', { type: 'module' });

const input = `
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

function render() {
    const canvas = document.getElementById('canvas');
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
