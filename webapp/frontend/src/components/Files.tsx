import { Tabs } from '@mantine/core';
import classes from './Files.module.scss';
import { projectStore } from '../stores/store';
import type { JSX } from 'react';
import type { WorkingFile } from '../types';
import { For } from '@preact/signals-react/utils';
import { ImageViewer } from './ImageViewer';
import { FileEditor } from './FileEditor';
import { signal } from '@preact/signals-react';

export function Files(): JSX.Element | null {
    const handleTabChange = (newValue: string | null): void => {
        projectStore.selectedTab.value = newValue ?? projectStore.files.value[0].filename;
    };

    if (projectStore.files.value.length === 0) {
        return null;
    }

    return (
        <Tabs
            value={projectStore.selectedTab.value ?? projectStore.files.value[0].filename}
            onChange={handleTabChange}
            className={classes.tabs}
        >
            <Tabs.List>
                <For each={projectStore.files}>
                    {(file) => (
                        <Tabs.Tab key={file.filename} value={file.filename}>
                            <div className={classes.tabFilename}>
                                {file.filename}
                                <div className={classes.unsavedIndicator}>
                                    {file.type === 'text' && file.contents != file.originalContents ? '*' : ' '}
                                </div>
                            </div>
                        </Tabs.Tab>
                    )}
                </For>
            </Tabs.List>

            <For each={projectStore.files}>
                {(file) => (
                    <Tabs.Panel key={file.filename} value={file.filename} className={classes.tabPanel}>
                        <File file={file} />
                    </Tabs.Panel>
                )}
            </For>
        </Tabs>
    );
}

interface FileProps {
    file: WorkingFile;
}

function File({ file }: FileProps): JSX.Element {
    if (file.type === 'binary' && file.contentType.startsWith('image/')) {
        const fileSignal = signal(file);
        return <ImageViewer file={fileSignal} />;
    }

    if (file.type === 'text') {
        return <FileEditor file={file} />;
    }

    return <div>Unsupported: {file.contentType}</div>;
}
