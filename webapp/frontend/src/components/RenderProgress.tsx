import { Progress } from '@mantine/core';
import { type JSX } from 'react';
import classes from './RenderProgress.module.scss';
import { formatDuration } from '../utils/time';
import { Signal, useSignal, useSignalEffect } from '@preact/signals-react';

export interface RenderProgressProps {
    progress: Signal<number>;
    working: Signal<boolean>;
    startTime: Signal<Date | undefined>;
}

export function RenderProgress({ progress, startTime, working }: RenderProgressProps): JSX.Element | null {
    const endTime = useSignal<Date | undefined>(undefined);

    useSignalEffect(() => {
        if (!working.value) {
            return;
        }

        setTimeout(() => {
            endTime.value = new Date();
        });
        const interval = setInterval(() => {
            endTime.value = new Date();
        }, 1000);

        return (): void => {
            clearInterval(interval);
            endTime.value = new Date();
        };
    });

    if (!startTime.value) {
        return null;
    }

    const progressPercentStr = (progress.value * 100.0).toFixed(0);

    let durationStr = '';
    let etaStr = '';
    if (endTime.value) {
        const duration = endTime.value.getTime() - startTime.value.getTime();
        durationStr = formatDuration(duration);

        if (working.value && progress.value > 0.0) {
            const estimatedTotalTime = duration / progress.value;
            const eta = Math.max(0, estimatedTotalTime - duration);
            etaStr = `(eta ${formatDuration(eta)})`;
        }
    }

    const progressLabel = `${progressPercentStr}% ${durationStr} ${etaStr}`;

    return (
        <Progress.Root radius="xs" size={30} className={classes.progressRoot}>
            <Progress.Section value={progress.value * 100.0} />
            <Progress.Section value={0.0001} className={classes.label}>
                <Progress.Label>{progressLabel}</Progress.Label>
            </Progress.Section>
        </Progress.Root>
    );
}
