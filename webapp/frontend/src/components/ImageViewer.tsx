import { type JSX } from 'react';
import type { ImageWorkingFile } from '../types';
import { CanvasViewer, type CanvasViewerHandle } from './CanvasViewer';
import { Signal, useSignal, useSignalEffect } from '@preact/signals-react';
import { useSignalRef } from '@preact/signals-react/utils';
import { drawArrayToCanvas as drawImageWorkingFileToCanvas } from '../utils/canvas';

export interface ImageViewerProps {
    file: Signal<ImageWorkingFile>;
}

export function ImageViewer({ file }: ImageViewerProps): JSX.Element {
    const canvasViewerRef = useSignalRef<CanvasViewerHandle | null>(null);
    const width = useSignal(300);
    const height = useSignal(300);

    useSignalEffect(() => {
        width.value = file.value.width;
        height.value = file.value.height;
        canvasViewerRef.current?.render((ctx) => {
            drawImageWorkingFileToCanvas(ctx, file.value);
        });
    });

    return <CanvasViewer ref={canvasViewerRef} width={width.value} height={height.value} />;
}
