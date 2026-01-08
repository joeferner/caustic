import type { ImageWorkingFile } from '../types';

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

export interface ImageData {
    width: number;
    height: number;
    data: ImageDataArray;
}

export function getImageDataFromBlob(blob: Blob): Promise<ImageData> {
    return new Promise<ImageData>((resolve, reject) => {
        const img = new Image();
        const url = URL.createObjectURL(blob);

        img.onload = (): void => {
            URL.revokeObjectURL(url);

            try {
                // Create a canvas to extract pixel data
                const canvas = document.createElement('canvas');
                canvas.width = img.width;
                canvas.height = img.height;

                const ctx = canvas.getContext('2d');
                if (!ctx) {
                    reject(new Error('Failed to get canvas context'));
                    return;
                }

                // Draw the image onto the canvas
                ctx.drawImage(img, 0, 0);

                // Get the pixel data (RGBA format)
                const imageData = ctx.getImageData(0, 0, img.width, img.height);
                const rgbaData = imageData.data;

                resolve({
                    width: img.width,
                    height: img.height,
                    data: rgbaData,
                });
            } catch (error) {
                reject(new Error(`failed to extract image data: ${error}`));
            }
        };

        img.onerror = (): void => {
            URL.revokeObjectURL(url);
            reject(new Error('Failed to load image'));
        };

        img.src = url;
    });
}

export function drawArrayToCanvas(ctx: CanvasRenderingContext2D, file: ImageWorkingFile): void {
    const imageData = ctx.createImageData(file.width, file.height);
    imageData.data.set(file.pixels);
    ctx.putImageData(imageData, 0, 0);
}
