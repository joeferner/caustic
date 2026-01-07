import type { CameraInfo, Color, LoadResults, WasmCodeResource, WasmResourceResolver } from './wasm/caustic_wasm';
import init, { load_openscad, get_camera_info, render } from './wasm/caustic_wasm.js';

export type { CameraInfo, Color };

export const initWasm = init;

export function loadOpenscad(resourceResolver: ResourceResolver): LoadResults {
    return load_openscad(resourceResolver);
}

export function getCameraInfo(): CameraInfo {
    return get_camera_info();
}

export function renderBlock(xmin: number, xmax: number, ymin: number, ymax: number): Color[] {
    return render(xmin, xmax, ymin, ymax);
}

export class ResourceResolver implements WasmResourceResolver {
    public constructor(private readonly main: CodeResource) {}

    public get_main(): WasmCodeResource {
        return this.main;
    }
}

export class CodeResource implements WasmCodeResource {
    public constructor(private readonly code: string) {}

    public get_code(): string {
        return this.code;
    }
}
