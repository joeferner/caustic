import { type JSX } from 'react';
import { projectStore } from '../stores/store';
import type { TextWorkingFile } from '../types';
import { Editor, type Monaco } from '@monaco-editor/react';
import { registerOpenscadLanguage } from '../monaco-openscad';
import type { editor } from 'monaco-editor';

interface FileEditorProps {
    file: TextWorkingFile;
}

export function FileEditor({ file }: FileEditorProps): JSX.Element {
    const handleCodeChange = (code: string | undefined): void => {
        projectStore.updateFile({ filename: file.filename, content: code ?? '' });
    };

    const handleEditorBeforeMount = (monaco: Monaco): void => {
        registerOpenscadLanguage(monaco);
    };

    const handleEditorMount = (monaco: editor.IStandaloneCodeEditor): void => {
        projectStore.registerEditor(file.filename, monaco);
    };

    return (
        <Editor
            height="100%"
            language="openscad"
            beforeMount={handleEditorBeforeMount}
            onMount={handleEditorMount}
            theme="vs-dark"
            value={file.contents}
            onChange={handleCodeChange}
            options={{ minimap: { enabled: false } }}
        />
    );
}
