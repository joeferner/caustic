/* eslint-disable react-refresh/only-export-components */

import { createContext, use, useRef, useState, type JSX, type ReactNode } from 'react';
import { getCameraInfo, initWasm, loadOpenscad, type CameraInfo } from './wasm';
import { RenderWorkerPool, type RenderCallbackFn } from './RenderWorkerPool';
import { Example, EXAMPLES } from './utils/examples';

export type UnsubscribeFn = () => void;

export interface RenderOptions {
    blockSize?: number;
    threadCount?: number;
}

export const DEFAULT_RENDER_BLOCK_SIZE = 50;

interface MyContextType {
    files: Record<string, string>;
    cameraInfo: CameraInfo | undefined;
    renderOptions: Required<RenderOptions>;
    render: () => Promise<void>;
    updateFile: (filename: string, content: string) => void;
    getFile: (filename: string) => string | undefined;
    subscribeToDrawEvents: (listener: RenderCallbackFn) => UnsubscribeFn;
}

const MyContext = createContext<MyContextType | undefined>(undefined);

interface MyProviderProps {
    children: ReactNode;
}

const renderWorkerPool = new RenderWorkerPool();

export function MyProvider({ children }: MyProviderProps): JSX.Element {
    const [renderOptions, _setRenderOptions] = useState<Required<RenderOptions>>({
        blockSize: DEFAULT_RENDER_BLOCK_SIZE,
        threadCount: navigator.hardwareConcurrency ?? 4,
    });
    const [files, setFiles] = useState<Record<string, string>>({
        'main.scad': EXAMPLES[Example.Car],
    });
    const [cameraInfo, setCameraInfo] = useState<CameraInfo | undefined>(undefined);
    const drawEventListeners = useRef(new Set<RenderCallbackFn>());

    const updateFile = (filename: string, content: string): void => {
        setFiles((prev) => ({
            ...prev,
            [filename]: content,
        }));
    };

    const getFile = (filename: string): string | undefined => {
        return files[filename];
    };

    const render = async (): Promise<void> => {
        const input = files['main.scad'];

        await initWasm();
        loadOpenscad(input);

        const cameraInfo = getCameraInfo();
        const { threadCount } = renderOptions;
        console.log(`Begin render ${cameraInfo.width}x${cameraInfo.height}`);
        setCameraInfo(cameraInfo);

        const localDrawEventListeners = drawEventListeners.current;
        renderWorkerPool.render(threadCount, input, {
            ...cameraInfo,
            ...renderOptions,
            callback: (event) => {
                for (const localDrawEventListener of localDrawEventListeners) {
                    localDrawEventListener(event);
                }
            },
        });
    };

    const subscribeToDrawEvents = (listener: RenderCallbackFn): UnsubscribeFn => {
        drawEventListeners.current.add(listener);
        return () => drawEventListeners.current.delete(listener);
    };

    const value: MyContextType = {
        files,
        cameraInfo,
        renderOptions,
        updateFile,
        getFile,
        render,
        subscribeToDrawEvents,
    };

    return <MyContext value={value}>{children}</MyContext>;
}

export function useMyContext(): MyContextType {
    const context = use(MyContext);
    if (!context) {
        throw new Error('useMyContext must be used within MyProvider');
    }
    return context;
}
