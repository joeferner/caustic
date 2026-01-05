import type { JSX } from 'react';
import { projectStore } from '../stores/store';
import type { TextWorkingFile } from '../types';
import { Editor } from '@monaco-editor/react';
import { registerOpenscadLanguage } from '../monaco-openscad';

interface FileEditorProps {
    file: TextWorkingFile;
}

export function FileEditor({ file }: FileEditorProps): JSX.Element {
    const handleCodeChange = (code: string | undefined): void => {
        projectStore.updateFile({ filename: file.filename, content: code ?? '' });
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
