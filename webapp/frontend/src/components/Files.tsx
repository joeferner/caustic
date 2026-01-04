import { Tabs } from '@mantine/core';
import { Editor } from '@monaco-editor/react';
import classes from './Files.module.scss';
import { store } from '../store';
import type { JSX } from 'react';
import { registerOpenscadLanguage } from '../monaco-openscad';
import type { WorkingFile } from '../types';
import { For } from '@preact/signals-react/utils';

export function Files(): JSX.Element | null {
    if (store.files.value.length === 0) {
        return null;
    }

    return (
        <Tabs value={store.files.value[0].filename} className={classes.tabs}>
            <Tabs.List>
                <For each={store.files}>
                    {(file) => (
                        <Tabs.Tab key={file.filename} value={file.filename}>
                            <div className={classes.tabFilename}>
                                {file.filename}
                                <div className={classes.unsavedIndicator}>
                                    {file.contents != file.originalContents ? '*' : ' '}
                                </div>
                            </div>
                        </Tabs.Tab>
                    )}
                </For>
            </Tabs.List>

            <For each={store.files}>
                {(file) => (
                    <Tabs.Panel key={file.filename} value={file.filename} className={classes.tabPanel}>
                        <FileEditor file={file} />
                    </Tabs.Panel>
                )}
            </For>
        </Tabs>
    );
}

function FileEditor({ file }: { file: WorkingFile }): JSX.Element {
    const handleCodeChange = (code: string | undefined): void => {
        store.updateFile({ filename: file.filename, content: code ?? '' });
    };

    return (
        <Editor
            height="100%"
            language="openscad"
            beforeMount={(monaco) => {
                registerOpenscadLanguage(monaco);
            }}
            theme="vs-dark"
            value={file.contents}
            onChange={handleCodeChange}
            options={{ minimap: { enabled: false } }}
        />
    );
}
