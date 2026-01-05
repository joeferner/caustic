import { type JSX } from 'react';
import type { BinaryWorkingFile } from '../types';
import { CanvasViewer, type CanvasViewerHandle } from './CanvasViewer';
import { Signal, useSignal, useSignalEffect } from '@preact/signals-react';
import { useSignalRef } from '@preact/signals-react/utils';
import { drawBlobToCanvas, getImageDimensionsOfBlob } from '../utils/canvas';

export interface ImageViewerProps {
    file: Signal<BinaryWorkingFile>;
}

export function ImageViewer({ file }: ImageViewerProps): JSX.Element {
    const canvasViewerRef = useSignalRef<CanvasViewerHandle | null>(null);
    const width = useSignal(300);
    const height = useSignal(300);

    useSignalEffect(() => {
        void (async (): Promise<void> => {
            const dims = await getImageDimensionsOfBlob(file.value.contents);
            width.value = dims.width;
            height.value = dims.height;
            canvasViewerRef.current?.render((ctx) => {
                void drawBlobToCanvas(ctx, file.value.contents);
            });
        })();
    });

    return <CanvasViewer ref={canvasViewerRef} width={width.value} height={height.value} />;
}
