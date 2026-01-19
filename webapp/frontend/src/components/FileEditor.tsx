import { type JSX } from 'react';
import { projectStore } from '../stores/store';
import type { TextWorkingFile } from '../types';
import { Editor, type Monaco } from '@monaco-editor/react';
import { type editor } from 'monaco-editor';
import 'vscode/localExtensionHost';
import { LANGUAGE_ID } from '../monaco-openscad';

interface FileEditorProps {
    file: TextWorkingFile;
}

export function FileEditor({ file }: FileEditorProps): JSX.Element {
    const filePath = `file:///workspace/${file.filename}`;

    const handleCodeChange = (code: string | undefined): void => {
        projectStore.updateFile({ filename: file.filename, content: code ?? '' });
    };

    const handleEditorMount = (editor: editor.IStandaloneCodeEditor, _monaco: Monaco): void => {
        projectStore.registerEditor(file.filename, editor);
    };

    return (
        <Editor
            height="100%"
            onMount={handleEditorMount}
            path={filePath}
            language={LANGUAGE_ID}
            theme="vs-dark"
            value={file.contents}
            onChange={handleCodeChange}
            options={{ minimap: { enabled: false } }}
        />
    );
}
