import { Tabs } from '@mantine/core';
import { Editor } from '@monaco-editor/react';
import classes from './Files.module.scss';
import { filesAtom, projectAtom, updateFileAtom } from '../store';
import type { JSX } from 'react';
import { registerOpenscadLanguage } from '../monaco-openscad';
import { useAtomValue, useSetAtom } from 'jotai';

export function Files(): JSX.Element {
    const project = useAtomValue(projectAtom);
    const files = useAtomValue(filesAtom);
    const updateFile = useSetAtom(updateFileAtom);

    const readOnly = project?.readOnly ?? true;

    return (
        <Tabs defaultValue="main.scad" className={classes.tabs}>
            <Tabs.List>
                {files.map((file) => {
                    return (
                        <Tabs.Tab key={file.filename} value={file.filename}>
                            {file.filename}
                            {readOnly ? ' [read only]' : null}
                        </Tabs.Tab>
                    );
                })}
            </Tabs.List>

            {files.map((file) => {
                return (
                    <Tabs.Panel key={file.filename} value={file.filename} className={classes.tabPanel}>
                        <Editor
                            height="100%"
                            language="openscad"
                            beforeMount={(monaco) => {
                                registerOpenscadLanguage(monaco);
                            }}
                            theme="vs-dark"
                            value={file.contents}
                            onChange={(code) => {
                                updateFile({ filename: file.filename, content: code ?? '' });
                            }}
                            options={{ minimap: { enabled: false }, readOnly }}
                        />
                    </Tabs.Panel>
                );
            })}
        </Tabs>
    );
}
