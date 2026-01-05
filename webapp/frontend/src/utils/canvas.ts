export function renderEmpty(ctx: CanvasRenderingContext2D, blockSize: number): void {
    for (let row = 0; ; row++) {
        const y = row * blockSize;
        if (y > ctx.canvas.height) {
            break;
        }
        for (let col = 0; ; col++) {
            const x = col * blockSize;
            if (x > ctx.canvas.width) {
                break;
            }
            const isWhite = (row + col) % 2 === 0;
            ctx.fillStyle = isWhite ? '#ffffff' : '#cccccc';
            ctx.fillRect(x, y, blockSize, blockSize);
        }
    }
}

export interface ImageDimensions {
    width: number;
    height: number;
}

export function getImageDimensionsOfBlob(blob: Blob): Promise<ImageDimensions> {
    return new Promise<ImageDimensions>((resolve, reject) => {
        const img = new Image();
        const url = URL.createObjectURL(blob);

        img.onload = (): void => {
            // Clean up the object URL after loading
            URL.revokeObjectURL(url);
            resolve({ width: img.width, height: img.height });
        };

        img.onerror = (): void => {
            URL.revokeObjectURL(url);
            reject(new Error('Failed to load image'));
        };

        img.src = url;
    });
}

export function drawBlobToCanvas(ctx: CanvasRenderingContext2D, blob: Blob): Promise<void> {
    return new Promise((resolve, reject) => {
        const img = new Image();
        const url = URL.createObjectURL(blob);

        img.onload = (): void => {
            ctx.drawImage(img, 0, 0);
            URL.revokeObjectURL(url);
            resolve();
        };

        img.onerror = (): void => {
            URL.revokeObjectURL(url);
            reject(new Error('Failed to load image'));
        };

        img.src = url;
    });
}
