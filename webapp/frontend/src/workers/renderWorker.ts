import type {
    InitImageData,
    RenderRequest,
    RenderRequestInit,
    RenderRequestWork,
    RenderResponseData,
    RenderResponseInit,
} from '../types';
import { initWasm, loadOpenscad, renderBlock } from '../wasm';
import { ImageData } from '../wasm/caustic_wasm';

let workerId = -1;
let imageData: Record<string, InitImageData> = {};

self.onmessage = (e: MessageEvent<RenderRequest>): void => {
    const { type } = e.data;

    if (type === 'init') {
        void init(e.data);
    } else if (type === 'work') {
        work(e.data);
    } else {
        throw new Error(`Unhandled message type: ${type}`);
    }
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any,@typescript-eslint/no-unsafe-member-access
(self as any).load_image = (name: string): ImageData => {
    const data = imageData?.[name];
    if (!data) {
        return new ImageData(100, 100, new Uint8Array([]));
    } else {
        return new ImageData(data.width, data.height, data.pixels);
    }
};

async function init(data: RenderRequestInit): Promise<void> {
    workerId = data.workerId;
    imageData = data.imageData;

    console.log(`[${workerId}] initializing worker`);
    await initWasm();
    loadOpenscad(data.input);

    const resultsMessage: RenderResponseInit = { type: 'init', workerId };
    self.postMessage(resultsMessage);
}

function work(data: RenderRequestWork): void {
    const { xmin, xmax, ymin, ymax } = data;

    const results = renderBlock(xmin, xmax, ymin, ymax);

    const resultsMessage: RenderResponseData = {
        type: 'data',
        workerId,
        xmin,
        xmax,
        ymin,
        ymax,
        data: results,
    };
    self.postMessage(resultsMessage);
}

export {};
