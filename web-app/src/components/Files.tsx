import { Tabs } from '@mantine/core';
import { Editor } from '@monaco-editor/react';
import styles from './Files.module.scss';
import { useMyContext } from '../state';
import type { JSX } from 'react';
import { registerOpenscadLanguage } from '../monaco-openscad';

export function Files(): JSX.Element {
    const { updateFile, files } = useMyContext();

    return (
        <Tabs defaultValue="main.scad" className={styles.tabs}>
            <Tabs.List>
                {files.map((file) => {
                    return (
                        <Tabs.Tab key={file.filename} value={file.filename}>
                            {file.filename}
                        </Tabs.Tab>
                    );
                })}
            </Tabs.List>

            {files.map((file) => {
                return (
                    <Tabs.Panel key={file.filename} value={file.filename} className={styles.tabPanel}>
                        <Editor
                            height="100%"
                            language="openscad"
                            beforeMount={(monaco) => {
                                registerOpenscadLanguage(monaco);
                            }}
                            theme="vs-dark"
                            value={file.contents}
                            onChange={(code) => {
                                updateFile(file.filename, code ?? '');
                            }}
                            options={{ minimap: { enabled: false } }}
                        />
                    </Tabs.Panel>
                );
            })}
        </Tabs>
    );
}
