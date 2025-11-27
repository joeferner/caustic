const BLOCK_SIZE = 10;
const worker = new Worker('worker.js', { type: 'module' });

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

function render() {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');

    for (let y = 0; y < canvas.height; y += BLOCK_SIZE) {
        for (let x = 0; x < canvas.width; x += BLOCK_SIZE) {
            worker.postMessage({
                width: canvas.width,
                height: canvas.height,
                xmin: x,
                xmax: Math.min(canvas.width, x + BLOCK_SIZE),
                ymin: y,
                ymax: Math.min(canvas.height, y + BLOCK_SIZE),
            });
        }
    }
}
