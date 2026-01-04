import React, { type JSX } from 'react';
import { MiniMap, TransformComponent, TransformWrapper, type ReactZoomPanPinchHandlers } from 'react-zoom-pan-pinch';
import classes from './Render.module.scss';
import { Button, Tooltip } from '@mantine/core';
import { ZoomIn as ZoomInIcon, ZoomOut as ZoomOutIcon, X as ResetZoomIcon } from 'react-bootstrap-icons';
import type { RenderResult } from '../types';
import * as _ from 'radash';
import { RenderProgress } from './RenderProgress';
import { useSignal, useSignalEffect } from '@preact/signals-react';
import { useSignalRef } from '@preact/signals-react/utils';
import { projectStore } from '../stores/store';

export function Render(): JSX.Element {
    const canvasRef = useSignalRef<HTMLCanvasElement | null>(null);
    const canvasMiniRef = useSignalRef<HTMLCanvasElement | null>(null);
    const showMinimap = useSignal(false);
    const progress = useSignal(1.0);
    const working = useSignal(false);
    const startTime = useSignal<Date | undefined>(undefined);

    // update empty background if block size changes
    useSignalEffect(() => {
        renderEmpty(canvasRef, projectStore.renderOptions.value.blockSize);
        renderEmpty(canvasMiniRef, projectStore.renderOptions.value.blockSize);
    });

    // subscribe to draw events to render
    useSignalEffect(() => {
        const blockSize = projectStore.renderOptions.value.blockSize;
        const _canvasRef = canvasRef;
        const _canvasMiniRef = canvasMiniRef;

        const unsubscribe = projectStore.subscribeToDrawEvents((event) => {
            if (event.type === 'init') {
                progress.value = 0.0;
                startTime.value = event.startTime;
                working.value = true;
                renderEmpty(_canvasRef, blockSize);
                renderEmpty(_canvasMiniRef, blockSize);
            } else if (event.type === 'renderResult') {
                progress.value = event.progress;
                if (event.progress >= 1.0) {
                    working.value = false;
                }
                renderDrawEvent(_canvasRef, event);
                renderDrawEvent(_canvasMiniRef, event);
            }
        });

        return unsubscribe;
    });

    const handleOnZoom = (): void => {
        const canvas = canvasRef.current;
        if (!canvas) {
            return;
        }

        const el = canvas.parentElement;
        const wrapperEl = el?.parentElement;
        if (!el || !wrapperEl) {
            return;
        }

        const elRect = el.getBoundingClientRect();
        const wrapperElRect = wrapperEl.getBoundingClientRect();

        const offScreen =
            elRect.y - wrapperElRect.y < 0 ||
            elRect.x - wrapperElRect.x < 0 ||
            wrapperElRect.right - elRect.right < 0 ||
            wrapperElRect.bottom - elRect.bottom < 0;
        showMinimap.value = offScreen;
    };

    return (
        <div className={classes.wrapper}>
            <TransformWrapper onZoom={handleOnZoom}>
                {(utils) => (
                    <React.Fragment>
                        <div className={classes.miniMap} style={{ display: showMinimap.value ? 'block' : 'none' }}>
                            <MiniMap width={150} height={150}>
                                <canvas
                                    ref={canvasMiniRef}
                                    width={projectStore.cameraInfo.value?.width ?? 500}
                                    height={projectStore.cameraInfo.value?.height ?? 500}
                                />
                            </MiniMap>
                        </div>
                        <Controls {...utils} />
                        <TransformComponent>
                            <canvas
                                className={classes.canvas}
                                ref={canvasRef}
                                width={projectStore.cameraInfo.value?.width ?? 500}
                                height={projectStore.cameraInfo.value?.height ?? 500}
                            />
                        </TransformComponent>
                    </React.Fragment>
                )}
            </TransformWrapper>
            <RenderProgress progress={progress} startTime={startTime} working={working} />
        </div>
    );
}

function Controls(options: ReactZoomPanPinchHandlers): JSX.Element {
    const handleZoomInClick = (): void => {
        options.zoomIn();
    };

    const handleZoomOutClick = (): void => {
        options.zoomOut();
    };

    const handleResetZoomClick = (): void => {
        options.resetTransform();
    };

    return (
        <div className={classes.controls}>
            <Tooltip label="Zoom In">
                <Button onClick={handleZoomInClick}>
                    <ZoomInIcon />
                </Button>
            </Tooltip>
            <Tooltip label="Zoom Out">
                <Button onClick={handleZoomOutClick}>
                    <ZoomOutIcon />
                </Button>
            </Tooltip>
            <Tooltip label="Reset Zoom">
                <Button onClick={handleResetZoomClick}>
                    <ResetZoomIcon />
                </Button>
            </Tooltip>
        </div>
    );
}

function getCanvasCtx(canvasRef: React.RefObject<HTMLCanvasElement | null>): CanvasRenderingContext2D | undefined {
    const canvas = canvasRef.current;
    if (!canvas) {
        return undefined;
    }
    const ctx = canvas.getContext('2d');
    if (!ctx) {
        console.error('could not get canvas context');
        return undefined;
    }
    return ctx;
}

function renderEmpty(canvasRef: React.RefObject<HTMLCanvasElement | null>, blockSize: number): void {
    const ctx = getCanvasCtx(canvasRef);
    if (!ctx) {
        return;
    }

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

function renderDrawEvent(canvasRef: React.RefObject<HTMLCanvasElement | null>, event: RenderResult): void {
    const ctx = getCanvasCtx(canvasRef);
    if (!ctx) {
        return;
    }

    const { xmin, xmax, ymin, ymax, data } = event;
    let i = 0;
    for (let y = ymin; y < ymax; y++) {
        for (let x = xmin; x < xmax; x++) {
            const color = data[i++];
            const { r, g, b } = color;
            ctx.fillStyle = `rgb(${r},${g},${b})`;
            ctx.fillRect(x, y, 1, 1);
        }
    }
}
